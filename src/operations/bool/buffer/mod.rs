use crate::prelude::BoolOperation;
use crate::prelude::BoolValue;
use anyhow::Error as AnyError;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Buffer {
  #[derivative(Debug = "ignore")]
  value: Box<dyn BoolValue>,
}

#[typetag::serde]
impl BoolOperation for Buffer {
  fn execute(&self) -> Result<Box<dyn BoolValue>, AnyError> {
    Ok(Box::new(self.value.evaluate()?))
  }
}

operation_value_impl!(BoolValue, Buffer, bool);

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_execute() {
    let operation = Buffer { value: Box::new(true) };
    assert!(operation.execute().unwrap().evaluate().unwrap());
  }

  #[test]
  fn test_serde() {
    let operation = &Buffer { value: Box::new(true) } as &dyn BoolOperation;
    let serialized = serde_yaml::to_string(operation).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Buffer
value:
  type: Bool
  value: true
    "#
      .trim()
    );
    let deserialized: Box<dyn BoolOperation> = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(
      operation.execute().unwrap().evaluate().unwrap(),
      deserialized.execute().unwrap().evaluate().unwrap()
    );
  }
}
