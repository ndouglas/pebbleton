use crate::prelude::Context;
use crate::prelude::FloatRangeValue;
use crate::prelude::FloatValue;
use crate::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct FloatInRange {
  #[derivative(Debug = "ignore")]
  pub value: Box<dyn FloatValue>,
  #[derivative(Debug = "ignore")]
  pub range: Box<dyn FloatRangeValue>,
}

#[typetag::serde]
impl Condition for FloatInRange {
  fn is_met(&self, _context: &dyn Context) -> Result<bool, AnyError> {
    let value = self.value.evaluate()?;
    let range = self.range.evaluate()?;
    Ok(range.contains(&value))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use core::ops::Range;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = FloatInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 0.0, end: 2.0 }),
    };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = FloatInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 1.0, end: 2.0 }),
    };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = FloatInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 0.0, end: 1.0 }),
    };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &FloatInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 0.0, end: 1.0 }),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: FloatInRange
value:
  type: Float
  value: 1.0
range:
  type: FloatRange
  start: 0.0
  end: 1.0
          "#
      .trim()
    );
    let deserialized: FloatInRange = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.value.evaluate().unwrap(), 1.0);
    assert_eq!(deserialized.range.evaluate().unwrap(), Range { start: 0.0, end: 1.0 });
  }
}
