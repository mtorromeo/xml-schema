use darling::FromDeriveInput;
use log::Level;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, FromDeriveInput)]
#[darling(attributes(xml_schema), forward_attrs(allow, doc, cfg))]
pub struct XmlSchemaAttributes {
  ident: syn::Ident,
  pub log_level: Option<String>,
  pub module_namespace_mappings: Option<String>,
  pub source: String,
  pub store_generated_code: Option<String>,
  pub target_prefix: Option<String>,
}

impl XmlSchemaAttributes {
  pub fn module_name(&self) -> String {
    self.ident.to_string()
  }

  pub fn log_level(&self) -> Level {
    match self.log_level.as_deref() {
      Some("error") => Level::Error,
      Some("warn") | Some("warning") => Level::Warn,
      Some("info") => Level::Info,
      Some("debug") => Level::Debug,
      Some("trace") => Level::Trace,
      Some(_) | None => Level::Warn,
    }
  }

  pub fn module_namespace_mappings(&self) -> BTreeMap<String, String> {
    let module_namespace_mappings = self.module_namespace_mappings.clone().unwrap_or_default();
    if module_namespace_mappings.is_empty() {
      return BTreeMap::default();
    }

    module_namespace_mappings
      .split(", ")
      .map(
        |mapping| match mapping.splitn(2, ": ").collect::<Vec<_>>().as_slice() {
          [first] => Some(("".to_owned(), first.to_string())),
          [first, second] => Some((first.to_string(), second.to_string())),
          _ => None,
        },
      )
      .flatten()
      .collect()
  }
}
