use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value {
    pub name: String,
    pub importance: f32,
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<ValueConflict>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConflict {
    pub value: String,
    pub resolution: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_value_without_conflicts() {
        let value = Value {
            name: "courage".to_string(),
            importance: 0.8,
            expression: "faces challenges".to_string(),
            conflicts: None,
        };

        let serialized = serde_json::to_value(&value).unwrap();
        assert!(!serialized.as_object().unwrap().contains_key("conflicts"));
    }

    #[test]
    fn test_value_deserialization() {
        let json = json!({
            "name": "integrity",
            "importance": 0.95,
            "expression": "maintains principles",
            "conflicts": [{
                "value": "loyalty",
                "resolution": "balance principles with loyalty"
            }]
        });

        let value: Value = serde_json::from_value(json).unwrap();
        assert_eq!(value.name, "integrity");
        assert_eq!(value.importance, 0.95);
        assert!(value.conflicts.is_some());
    }
}
