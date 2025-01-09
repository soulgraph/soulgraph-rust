use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    pub style: String,
    pub boundaries: Vec<Boundary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Boundary {
    pub r#type: String,
    pub description: String,
    pub enforcement: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_relationship_serialization() {
        let relationship = Relationship {
            style: "professional".to_string(),
            boundaries: vec![Boundary {
                r#type: "personal".to_string(),
                description: "Maintain professional distance".to_string(),
                enforcement: "strict".to_string(),
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
                "enforcement": "moderate"
            }]
        });

        let relationship: Relationship = serde_json::from_value(json).unwrap();
        assert_eq!(relationship.style, "friendly");
        assert_eq!(relationship.boundaries[0].r#type, "communication");
    }
}
