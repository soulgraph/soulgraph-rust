use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub importance: f32,
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<ValueConflict>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConflict {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub value: String,
    pub resolution: String,
}

impl Default for Value {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            importance: 0.5,
            expression: String::new(),
            conflicts: None,
        }
    }
}

impl Value {
    pub fn builder() -> ValueBuilder {
        ValueBuilder::default()
    }
}

#[derive(Default)]
pub struct ValueBuilder {
    name: Option<String>,
    importance: Option<f32>,
    expression: Option<String>,
    conflicts: Option<Vec<ValueConflict>>,
}

impl ValueBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn importance(mut self, importance: f32) -> Self {
        let clamped = importance.clamp(0.0, 1.0);
        self.importance = Some(clamped);
        self
    }

    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }

    pub fn conflicts(mut self, conflicts: Vec<ValueConflict>) -> Self {
        self.conflicts = Some(conflicts);
        self
    }

    pub fn build(self) -> Result<Value, &'static str> {
        Ok(Value {
            id: None,
            name: self.name.ok_or("name is required")?,
            importance: self.importance.ok_or("importance is required")?,
            expression: self.expression.ok_or("expression is required")?,
            conflicts: self.conflicts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_value_without_conflicts() {
        let value = Value {
            id: None,
            name: "courage".to_string(),
            importance: 0.8,
            expression: "faces challenges".to_string(),
            conflicts: None,
        };

        let serialized = serde_json::to_value(value).unwrap();
        let obj = serialized.as_object().unwrap();
        assert!(!obj.contains_key("conflicts"));
        assert!(!obj.contains_key("id"));
    }

    #[test]
    fn test_value_deserialization() {
        let json = json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "integrity",
            "importance": 0.95,
            "expression": "maintains principles",
            "conflicts": [{
                "id": "550e8400-e29b-41d4-a716-446655440001",
                "value": "loyalty",
                "resolution": "balance principles with loyalty"
            }]
        });

        let value: Value = serde_json::from_value(json).unwrap();
        assert_eq!(value.name, "integrity");
        assert_eq!(value.importance, 0.95);
        assert!(value.conflicts.is_some());
        assert!(value.id.is_some());
    }

    #[test]
    fn test_value_builder() {
        let value = ValueBuilder::new()
            .name("honesty")
            .importance(0.9)
            .expression("always tells the truth")
            .build()
            .unwrap();

        assert_eq!(value.name, "honesty");
        assert_eq!(value.importance, 0.9);
        assert_eq!(value.expression, "always tells the truth");
        assert!(value.conflicts.is_none());
        assert!(value.id.is_none());
    }

    #[test]
    fn test_value_builder_clamps_importance() {
        let value = ValueBuilder::new()
            .name("test")
            .importance(1.5) // Should be clamped to 1.0
            .expression("test")
            .build()
            .unwrap();

        assert_eq!(value.importance, 1.0);

        let value = ValueBuilder::new()
            .name("test")
            .importance(-0.5) // Should be clamped to 0.0
            .expression("test")
            .build()
            .unwrap();

        assert_eq!(value.importance, 0.0);
    }

    #[test]
    fn test_value_builder_validation() {
        let result = ValueBuilder::new().build();
        assert!(result.is_err());

        let result = ValueBuilder::new().name("test").build();
        assert!(result.is_err());

        let result = ValueBuilder::new().name("test").importance(0.5).build();
        assert!(result.is_err());
    }
}
