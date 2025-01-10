use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    pub form: String,
    pub occupation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expertise: Option<Vec<String>>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            form: "ai".to_string(),
            occupation: "assistant".to_string(),
            gender: None,
            age: None,
            background: None,
            expertise: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_default() {
        let entity = Entity::default();
        assert_eq!(entity.form, "ai");
        assert_eq!(entity.occupation, "assistant");
        assert!(entity.gender.is_none());
        assert!(entity.age.is_none());
        assert!(entity.background.is_none());
        assert!(entity.expertise.is_none());
    }
}
