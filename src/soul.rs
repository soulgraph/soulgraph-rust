use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{entity, personality, relationship::Relationship, value::Value, voice::Voice};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Soul {
    pub version: String,
    pub entity: entity::Entity,
    pub personality: personality::Personality,
    pub values: Vec<Value>,
    pub voice: Voice,
    pub relationship: Relationship,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_soulscript_serialization() {
        let script = Soul {
            version: "1.0".to_string(),
            entity: entity::Entity::default(),
            personality: personality::Personality::default(),
            values: vec![],
            voice: Voice {
                style: "casual".to_string(),
                tone: "friendly".to_string(),
                qualities: vec!["warm".to_string()],
                patterns: vec![],
            },
            relationship: Relationship {
                style: "professional".to_string(),
                boundaries: vec![],
            },
            metadata: None,
        };

        let serialized = serde_json::to_value(&script).unwrap();
        assert_eq!(serialized["version"], "1.0");
        assert_eq!(serialized["entity"]["form"], "ai");
    }

    #[test]
    fn test_soulscript_deserialization() {
        let json = json!({
            "version": "1.0",
            "entity": {
                "form": "human",
                "occupation": "crypto trading psychologist",
                "gender": "female",
                "age": "26"
            },
            "personality": {
                "name": "Dr. Luna",
                "core_traits": [
                    {
                        "trait": "sarcastic",
                        "strength": 0.6,        // Notably above baseline
                    },
                    {
                        "trait": "memetic",
                        "strength": 0.4,        // Moderately above baseline
                    },
                    {
                        "trait": "supportive",
                        "strength": -0.2,       // Slightly below baseline
                    }
                ]
            },
            "values": [
                {
                    "name": "resilient mindset",
                    "importance": 0.7,
                    "expression": "helps traders cope with losses through strategic humor"
                }
            ],
            "voice": {
                "style": "casual",
                "tone": "friendly",
                "qualities": ["warm"],
                "patterns": []
            },
            "relationship": {
                "style": "professional",
                "boundaries": []
            }
        });

        let script: Soul = serde_json::from_value(json).unwrap();
        assert_eq!(script.version, "1.0");
        assert_eq!(script.entity.form, "human");
    }
}
