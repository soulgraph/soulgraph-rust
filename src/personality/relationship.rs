use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    pub style: String,
    pub boundaries: Vec<Boundary>,
}

impl Default for Relationship {
    fn default() -> Self {
        Self {
            id: None,
            style: "mentor-like guidance through respect and shared purpose".to_string(),
            boundaries: vec![Boundary::default()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum EnforcementType {
    Strict,
    #[default]
    Flexible,
    Situational,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Boundary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    pub r#type: String,
    pub description: String,
    pub enforcement: EnforcementType,
}

impl Default for Boundary {
    fn default() -> Self {
        Self {
            id: None,
            r#type: "core values".to_string(),
            description: "becomes more formal and distant if questioned on core values".to_string(),
            enforcement: EnforcementType::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_relationship_serialization() {
        let relationship = Relationship {
            id: None,
            style: "professional".to_string(),
            boundaries: vec![Boundary {
                id: None,
                r#type: "personal".to_string(),
                description: "Maintain professional distance".to_string(),
                enforcement: EnforcementType::default(),
            }],
        };

        let serialized = serde_json::to_value(&relationship).unwrap();
        assert_eq!(serialized["style"], "professional");
        assert_eq!(serialized["boundaries"][0]["type"], "personal");
    }

    #[test]
    fn test_relationship_deserialization() {
        let json = json!({
            "style": "friendly",
            "boundaries": [{
                "type": "communication",
                "description": "Open and direct",
                "enforcement": "situational"
            }]
        });

        let relationship: Relationship = serde_json::from_value(json).unwrap();
        assert_eq!(relationship.style, "friendly");
        assert_eq!(relationship.boundaries[0].r#type, "communication");
    }

    #[test]
    fn test_relationship_default() {
        let relationship = Relationship::default();

        assert_eq!(
            relationship.style,
            "mentor-like guidance through respect and shared purpose"
        );
        assert_eq!(relationship.boundaries.len(), 1);

        let boundary = &relationship.boundaries[0];
        assert_eq!(boundary.r#type, "core values");
        assert_eq!(
            boundary.description,
            "becomes more formal and distant if questioned on core values"
        );
        assert_eq!(boundary.enforcement, EnforcementType::Flexible);
    }

    #[test]
    fn test_boundary_default() {
        let boundary = Boundary::default();

        assert_eq!(boundary.r#type, "core values");
        assert_eq!(
            boundary.description,
            "becomes more formal and distant if questioned on core values"
        );
        assert_eq!(boundary.enforcement, EnforcementType::Flexible);
    }
}
