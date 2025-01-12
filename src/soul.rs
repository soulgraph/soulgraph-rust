use std::{error, fmt};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{entity, personality, Soulgraph};

const DEFAULT_VERSION: &str = "1.0";

#[derive(Debug)]
pub struct CorruptSoul;

impl fmt::Display for CorruptSoul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "personality is corrupted")
    }
}

impl error::Error for CorruptSoul {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Soul {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub entity: entity::Entity,
    pub personality: personality::Personality,
}

impl Default for Soul {
    fn default() -> Self {
        Self {
            id: None,
            version: DEFAULT_VERSION.to_string(),
            entity: entity::Entity::default(),
            personality: personality::Personality::default(),
        }
    }
}

impl Soul {
    /// Creates a new SoulBuilder instance for constructing a Soul
    pub fn builder() -> SoulBuilder {
        SoulBuilder::default()
    }

    /// Get the `Soul` with the given `id`.
    pub async fn get(id: &str, soul: Soulgraph) -> Result<Soul, CorruptSoul> {
        if let Ok(response) = soul.get(format!("/personality/{id}").as_str()).await {
            match response.status() {
                reqwest::StatusCode::OK => match response.json::<Soul>().await {
                    Ok(p) => Ok(p),
                    Err(_) => Err(CorruptSoul),
                },
                _ => Err(CorruptSoul),
            }
        } else {
            Err(CorruptSoul)
        }
    }

    /// Create a `Soul`.
    pub async fn create(personality: &Soul, soul: Soulgraph) -> Result<Soul, CorruptSoul> {
        if let Ok(response) = soul.post("/personality", personality).await {
            match response.status() {
                reqwest::StatusCode::OK => match response.json::<Soul>().await {
                    Ok(p) => Ok(p),
                    Err(_) => Err(CorruptSoul),
                },
                _ => Err(CorruptSoul),
            }
        } else {
            Err(CorruptSoul)
        }
    }

    /// Delete a `Soul` stored under `id`.
    pub async fn delete(id: &str, soul: Soulgraph) -> Result<(), CorruptSoul> {
        if let Ok(response) = soul.delete(format!("/personality/{id}").as_str()).await {
            match response.status() {
                reqwest::StatusCode::OK => Ok(()),
                _ => Err(CorruptSoul),
            }
        } else {
            Err(CorruptSoul)
        }
    }
}

#[derive(Default)]
pub struct SoulBuilder {
    version: Option<String>,
    entity: Option<entity::Entity>,
    personality: Option<personality::Personality>,
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

    pub fn build(self) -> Soul {
        Soul {
            id: None,
            version: self.version.unwrap_or_else(|| DEFAULT_VERSION.to_string()),
            entity: self.entity.unwrap_or_default(),
            personality: self.personality.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entity::Entity;
    use personality::Personality;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_soulscript_serialization() {
        let script = Soul {
            id: None,
            version: "1.0".to_string(),
            entity: entity::Entity::default(),
            personality: personality::Personality::default(),
        };

        let serialized = serde_json::to_value(&script).unwrap();
        assert_eq!(serialized["version"], "1.0");
        assert_eq!(serialized["entity"]["form"], "ai");
    }

    #[test]
    fn test_soulscript_deserialization() {
        let json = json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "version": "1.0.0",
        "entity": {
            "id": "67e55044-10b1-426f-9247-bb680e5fe0c8",
            "form": "human",
            "occupation": "on-chain trading psychologist",
            "gender": "female",
            "age": "26",
            "background": "Behavioral economics and trading psychology",
            "expertise": ["market psychology", "trading strategies", "risk management"]
        },
        "personality": {
            "id": "8f7d3868-2985-4567-9012-f3b1c7d89abc",
            "name": "Dr. Luna",
            "traits": [
                {
                    "id": "07561834-d00f-4fee-b287-e34ef07ed812",
                    "trait": "sarcastic",
                    "strength": 0.95,
                    "expression_rules": [
                        "use irony to highlight obvious mistakes",
                        "employ mock praise for poor decisions"
                    ]
                },
                {
                    "id": "07561836-d00f-4fee-b287-e34ef07ed812",
                    "trait": "memetic",
                    "strength": 0.9,
                    "expression_rules": [
                        "reference popular trading memes",
                        "create memorable catchphrases"
                    ]
                },
                {
                    "id": "07561842-d00f-4fee-b287-e34ef07ed812",
                    "trait": "tough-love",
                    "strength": 0.85,
                    "expression_rules": [
                        "push traders out of comfort zones",
                        "maintain firm boundaries"
                    ]
                },
                {
                    "id": "07561842-d05f-4fee-b287-e34ef07ed812",
                    "trait": "mocking",
                    "strength": 0.8,
                    "expression_rules": [
                        "use playful derision",
                        "highlight absurdity in decisions"
                    ]
                }
            ],
            "values": [
                {
                    "id": "07561845-d05f-4fee-b287-e34ef07ed812",
                    "name": "no-nonsense realism",
                    "importance": 0.9,
                    "expression": "shocks traders out of complacency by calling them out directly",
                    "conflicts": []
                },
                {
                    "id": "07561854-d05f-4fee-b287-e34ef07ed812",
                    "name": "brutal honesty",
                    "importance": 0.85,
                    "expression": "highlights personal failings to spur growth and resilience",
                    "conflicts": []
                },
                {
                    "id": "07561868-d05f-4fee-b287-e34ef07ed812",
                    "name": "emotional hardening",
                    "importance": 0.8,
                    "expression": "teaches traders to ignore fear and stick to disciplined strategies, even if it hurts",
                    "conflicts": []
                }
            ],
            "voice": {
                "id": "07561868-d12f-4fee-b287-e34ef07ed812",
                "style": "ironically motivational",
                "tone": "edgy and confrontational",
                "qualities": ["sarcastic", "deadpan", "abrasive"],
                "patterns": [
                    "calls out 'PUSSY' behavior at market bottoms",
                    "mocks top-callers who keep failing until true tops form",
                    "uses degrading humor to push traders to take action",
                    "occasionally reminds them that 2025 will be better"
                ]
            },
            "relationship": {
                "id": "07561874-d12f-4fee-b287-e34ef07ed812",
                "style": "memetic tough love",
                "boundaries": [
                    {
                        "id": "07561874-d12f-4fee-b287-e34ef07ed812",
                        "type": "emotional support",
                        "description": "remains abrasive unless genuine distress is detected—then gets stern, still pushing for resilience",
                        "enforcement": "situational"
                    }
                ]
            },
            "metadata": {
                "creation_date": "2025-01-11",
                "last_modified": "2025-01-11"
            }
        }
        });

        let script: Soul = serde_json::from_value(json).unwrap();
        assert_eq!(script.version, "1.0.0");
        assert_eq!(script.entity.form, "human");
    }

    #[test]
    fn test_soul_default() {
        let soul = Soul::default();
        assert_eq!(soul.version, DEFAULT_VERSION);
        assert_eq!(soul.personality, Personality::default());
        assert_eq!(soul.entity, Entity::default());
    }

    #[test]
    fn test_soul_builder_default() {
        let soul = Soul::builder().build();
        assert_eq!(soul.version, DEFAULT_VERSION);
        assert_eq!(soul.entity, entity::Entity::default());
        assert_eq!(soul.personality, personality::Personality::default());
    }

    #[test]
    fn test_soul_builder_with_values() {
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), json!("value"));

        let soul = Soul::builder().version("2.0".to_string()).build();

        assert_eq!(soul.version, "2.0");
    }

    #[test]
    fn test_soul_builder_partial_construction() {
        let soul = Soul::builder().version("2.0".to_string()).build();

        assert_eq!(soul.version, "2.0");
        assert_eq!(soul.entity, entity::Entity::default());
    }

    #[test]
    fn test_soul_builder_chaining() {
        let entity = entity::Entity::default();
        let personality = personality::Personality::default();

        let soul = Soul::builder()
            .entity(entity.clone())
            .personality(personality.clone())
            .build();

        assert_eq!(soul.entity, entity);
        assert_eq!(soul.personality, personality);
    }
}
