use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{entity, personality, relationship::Relationship, value::Value, voice::Voice};

const DEFAULT_VERSION: &str = "1.0";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Soul {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub entity: entity::Entity,
    pub personality: personality::Personality,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Relationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Default for Soul {
    fn default() -> Self {
        Self {
            id: None,
            version: DEFAULT_VERSION.to_string(),
            entity: entity::Entity::default(),
            personality: personality::Personality::default(),
            values: Some(Vec::new()),
            voice: Some(Voice::default()),
            relationship: Some(Relationship::default()),
            metadata: None,
        }
    }
}

impl Soul {
    pub fn builder() -> SoulBuilder {
        SoulBuilder::default()
    }
}

#[derive(Default)]
pub struct SoulBuilder {
    version: Option<String>,
    entity: Option<entity::Entity>,
    personality: Option<personality::Personality>,
    values: Option<Vec<Value>>,
    voice: Option<Voice>,
    relationship: Option<Relationship>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl SoulBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn entity(mut self, entity: entity::Entity) -> Self {
        self.entity = Some(entity);
        self
    }

    pub fn personality(mut self, personality: personality::Personality) -> Self {
        self.personality = Some(personality);
        self
    }

    pub fn values(mut self, values: Vec<Value>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn voice(mut self, voice: Voice) -> Self {
        self.voice = Some(voice);
        self
    }

    pub fn relationship(mut self, relationship: Relationship) -> Self {
        self.relationship = Some(relationship);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Soul {
        Soul {
            id: None,
            version: self.version.unwrap_or_else(|| DEFAULT_VERSION.to_string()),
            entity: self.entity.unwrap_or_default(),
            personality: self.personality.unwrap_or_default(),
            values: self.values,
            voice: self.voice,
            relationship: self.relationship,
            metadata: self.metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_soulscript_serialization() {
        let script = Soul {
            id: None,
            version: "1.0".to_string(),
            entity: entity::Entity::default(),
            personality: personality::Personality::default(),
            values: Some(vec![]),
            voice: Some(Voice {
                style: "casual".to_string(),
                tone: "friendly".to_string(),
                qualities: vec!["warm".to_string()],
                patterns: vec![],
            }),
            relationship: Some(Relationship {
                id: None,
                style: "professional".to_string(),
                boundaries: vec![],
            }),
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
                "traits": [
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

    #[test]
    fn test_soul_default() {
        let soul = Soul::default();
        assert_eq!(soul.version, DEFAULT_VERSION);
        assert_eq!(soul.values.unwrap().len(), 0);
        assert!(soul.metadata.is_none());
    }

    #[test]
    fn test_soul_builder_default() {
        let soul = Soul::builder().build();
        assert_eq!(soul.version, DEFAULT_VERSION);
        assert_eq!(soul.entity, entity::Entity::default());
        assert_eq!(soul.personality, personality::Personality::default());
        assert!(soul.values.is_none());
        assert!(soul.metadata.is_none());
    }

    #[test]
    fn test_soul_builder_with_values() {
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), json!("value"));

        let test_value = Value {
            name: "test value".to_string(),
            importance: 0.5,
            expression: "test expression".to_string(),
            conflicts: None,
        };

        let soul = Soul::builder()
            .version("2.0".to_string())
            .values(vec![test_value.clone()])
            .metadata(metadata.clone())
            .build();

        let soul_values = soul.values.unwrap();
        assert_eq!(soul.version, "2.0");
        assert_eq!(soul_values.len(), 1);
        assert_eq!(soul_values[0], test_value);
        assert_eq!(soul.metadata.unwrap().get("test").unwrap(), &json!("value"));
    }

    #[test]
    fn test_soul_builder_partial_construction() {
        let soul = Soul::builder().version("2.0".to_string()).build();

        assert_eq!(soul.version, "2.0");
        assert_eq!(soul.entity, entity::Entity::default());
        assert!(soul.values.is_none());
        assert!(soul.metadata.is_none());
    }

    #[test]
    fn test_soul_builder_chaining() {
        let entity = entity::Entity::default();
        let personality = personality::Personality::default();
        let voice = Voice {
            style: "professional".to_string(),
            tone: "formal".to_string(),
            qualities: vec!["articulate".to_string()],
            patterns: vec![],
        };

        let soul = Soul::builder()
            .entity(entity.clone())
            .personality(personality.clone())
            .voice(voice.clone())
            .build();

        assert_eq!(soul.entity, entity);
        assert_eq!(soul.personality, personality);
        assert_eq!(soul.voice.unwrap(), voice);
    }
}
