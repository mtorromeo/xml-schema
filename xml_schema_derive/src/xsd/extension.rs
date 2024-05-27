use crate::xsd::{
  attribute::Attribute, choice::Choice, group::Group, rust_types_mapping::RustTypesMapping, sequence::Sequence,
  Implementation, XsdContext,
};
use proc_macro2::TokenStream;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  root = "extension",
  prefix = "xs",
  namespace = "xs: http://www.w3.org/2001/XMLSchema"
)]
pub struct Extension {
  #[yaserde(attribute)]
  pub base: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "sequence")]
  pub sequences: Vec<Sequence>,
  #[yaserde(rename = "group")]
  pub group: Option<Group>,
  #[yaserde(rename = "choice")]
  pub choices: Vec<Choice>,
}

impl Implementation for Extension {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let mut rust_type = RustTypesMapping::get(context, &self.base);

    let attributes: TokenStream = self
      .attributes
      .iter()
      .map(|attribute| attribute.implement(namespace_definition, prefix, context))
      .collect();

    if context.extension_is_string(self.base.as_str()) {
      rust_type = quote!(String);
    // } else if matches!(self.base.as_str(), "normalizedString") && !self.attributes.is_empty() {
      // } else if matches!(self.base.as_str(), "normalizedString") && matches!(self.attributes.first().map(|a| a.kind.as_ref().map(|k| k.as_ref())), Some(Some("domain:statusValueType"))) {
      // rust_type = quote!(Option<String>);
    }

    let inner_attribute = if matches!(format!("{rust_type}").as_str(), "String" | "Option < String >") {
      quote!(#[yaserde(text)])
    } else {
      // quote!(#[yaserde(flatten)])
      TokenStream::new()
    };

    quote!(
      #inner_attribute
      pub base: #rust_type,
      #attributes
    )
  }
}

impl Extension {
  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    let rust_type = RustTypesMapping::get(context, &self.base);

    let group_content = self
      .group
      .as_ref()
      .map(|group| {
        let group_type = group.get_type_implementation(context, prefix);

        quote!(
          ,
          #[yaserde(flatten)]
          pub extension : #group_type
        )
      })
      .unwrap_or_default();

    quote!(
      #[yaserde(flatten)]
      pub base : #rust_type
      #group_content
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn extension() {
    let st = Extension {
      base: "xs:string".to_string(),
      attributes: vec![],
      sequences: vec![],
      group: None,
      choices: vec![],
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = st.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
        #[yaserde(text)]
        pub base: String,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  fn extension_with_attributes() {
    use crate::xsd::attribute::Required;

    let st = Extension {
      base: "xs:string".to_string(),
      attributes: vec![
        Attribute {
          name: Some("attribute_1".to_string()),
          kind: Some("xs:string".to_string()),
          reference: None,
          required: Required::Required,
          simple_type: None,
        },
        Attribute {
          name: Some("attribute_2".to_string()),
          kind: Some("xs:boolean".to_string()),
          reference: None,
          required: Required::Optional,
          simple_type: None,
        },
      ],
      sequences: vec![],
      group: None,
      choices: vec![],
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = st.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
        #[yaserde(text)]
        pub base: String,
        #[yaserde(attribute)]
        pub attribute_1: String,
        #[yaserde(attribute)]
        pub attribute_2: Option<bool> ,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }
}
