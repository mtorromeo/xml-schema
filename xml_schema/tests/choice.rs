extern crate yaserde_derive;

use xml_schema_derive::XmlSchema;
use yaserde::de::from_str;
use yaserde::ser::to_string;

/*
#[test]
fn choice() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/choice.xsd")]
  struct ChoiceTypeSchema;

  use choice_type_schema::xml_schema_types::*;

  let xml_1 = r#"<?xml version="1.0" encoding="UTF-8"?>
  <person>
    <firstname>John</firstname>
  </person>
  "#;

  let sample_1: Person = from_str(xml_1).unwrap();

  let model = Person::firstname{
    base: "John".to_string(),
    scope: None,
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><Person><firstname>John</firstname></Person>"#
  );
}

#[test]
fn choice_sequence() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/choice_sequence.xsd")]
  struct ChoiceTypeSchema;

  use choice_type_schema::xml_schema_types::*;

  let xml_1 = r#"<?xml version="1.0" encoding="UTF-8"?>
  <person>
    <name>Doe</name>
    <firstname>John</firstname>
    <firstname>John</firstname>
  </person>
  "#;

  let sample_1: Person = from_str(xml_1).unwrap();

  let model = Person {
    content: NamedEntity {
      name: "Doe".to_string(),
      named_entity_choice_1: NamedEntityChoice1::firstname {
        base: "John".to_string(),
        scope: None,
      },
      named_entity_choice_2: NamedEntityChoice2::firstname {
        base: "John".to_string(),
        scope: None,
      }
    }
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><Person><name>Doe</name><firstname>John</firstname></Person>"#
  );
}
*/

#[test]
fn choice_multiple() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/choice_multiple.xsd")]
  struct ChoiceTypeSchema;

  let xml_1 = r#"<?xml version="1.0" encoding="UTF-8"?>
  <gift>
    <priceHistory>1</priceHistory>
    <priceHistory>3</priceHistory>
  </gift>
  "#;

  let sample_1: Gift = from_str(xml_1).unwrap();

  let model = Gift {
    content: vec![Object::price_history(1), Object::price_history(3)],
  };
  
  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><Gift><Object><priceHistory>1</priceHistory><priceHistory>3</priceHistory></Object></Gift>"#
  );
}
