mod relationship;
mod traits;
mod value;
mod voice;

use relationship::Relationship;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error, fmt};
use traits::Trait;
use value::Value;
use voice::Voice;

use crate::Soulgraph;

#[derive(Debug)]
pub struct CorruptPersonality;

impl fmt::Display for CorruptPersonality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "personality is corrupted")
    }
}

impl error::Error for CorruptPersonality {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Personality {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub traits: Vec<Trait>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Relationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            id: None,
            name: "AI Assistant".to_string(),
            traits: vec![
                Trait {
                    id: None,
                    r#trait: "helpful".to_string(),
                    strength: 0.9,
                    expression_rules: Some(vec!["always seeks to assist".to_string()]),
                },
                Trait {
                    id: None,
                    r#trait: "professional".to_string(),
                    strength: 0.8,
                    expression_rules: Some(vec!["maintains appropriate boundaries".to_string()]),
                },
            ],
            values: None,
            voice: None,
            relationship: None,
            metadata: Some(HashMap::from([
                (
                    "creation_date".to_string(),
                    chrono::Utc::now().date_naive().to_string(),
                ),
                (
                    "last_modified".to_string(),
                    chrono::Utc::now().date_naive().to_string(),
                ),
            ])),
        }
    }
}

#[derive(Default)]
pub struct PersonalityBuilder {
    id: Option<uuid::Uuid>,
    name: Option<String>,
    traits: Vec<Trait>,
    values: Option<Vec<Value>>,
    voice: Option<Voice>,
    relationship: Option<Relationship>,
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq)]
pub enum PersonalityBuilderError {
    MissingName,
    NoTraits,
}

impl std::fmt::Display for PersonalityBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonalityBuilderError::MissingName => write!(f, "name is required"),
            PersonalityBuilderError::NoTraits => write!(f, "at least one trait is required"),
        }
    }
}

impl std::error::Error for PersonalityBuilderError {}

impl PersonalityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: uuid::Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn add_trait(mut self, trait_: Trait) -> Self {
        self.traits.push(trait_);
        self
    }

    pub fn add_value(mut self, value: Value) -> Self {
        if self.values.is_none() {
            self.values = Some(Vec::new());
        }
        if let Some(values) = self.values.as_mut() {
            values.push(value);
        }
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

    pub fn set_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        if let Some(metadata) = self.metadata.as_mut() {
            metadata.insert(key.into(), value.into());
        }
        self
    }

    pub fn build(self) -> Result<Personality, PersonalityBuilderError> {
        let name = self.name.ok_or(PersonalityBuilderError::MissingName)?;

        if self.traits.is_empty() {
            return Err(PersonalityBuilderError::NoTraits);
        }

        let mut metadata = self.metadata.unwrap_or_default();
        if !metadata.contains_key("creation_date") {
            metadata.insert(
                "creation_date".to_string(),
                chrono::Utc::now().date_naive().to_string(),
            );
        }
        metadata.insert(
            "last_modified".to_string(),
            chrono::Utc::now().date_naive().to_string(),
        );

        Ok(Personality {
            id: self.id,
            name,
            traits: self.traits,
            values: self.values,
            voice: self.voice,
            relationship: self.relationship,
            metadata: Some(metadata),
        })
    }
}

impl Personality {
    /// Creates a new PersonalityBuilder instance for constructing a Personality
    pub fn builder() -> PersonalityBuilder {
        PersonalityBuilder::new()
    }

    /// Get the `Personality` with the given `id`.
    pub async fn get(id: &str, soul: Soulgraph) -> Result<Personality, CorruptPersonality> {
        if let Ok(response) = soul.get(format!("/personality/{id}").as_str()).await {
            match response.status() {
                reqwest::StatusCode::OK => match response.json::<Personality>().await {
                    Ok(p) => Ok(p),
                    Err(_) => Err(CorruptPersonality),
                },
                _ => Err(CorruptPersonality),
            }
        } else {
            Err(CorruptPersonality)
        }
    }

    /// Create a `Personality`.
    pub async fn create(
        personality: &Personality,
        soul: Soulgraph,
    ) -> Result<Personality, CorruptPersonality> {
        if let Ok(response) = soul.post("/personality", personality).await {
            match response.status() {
                reqwest::StatusCode::OK => match response.json::<Personality>().await {
                    Ok(p) => Ok(p),
                    Err(_) => Err(CorruptPersonality),
                },
                _ => Err(CorruptPersonality),
            }
        } else {
            Err(CorruptPersonality)
        }
    }

    /// Update a `Personality` stored under `id`.
    pub async fn update(
        id: &str,
        personality: &Personality,
        soul: Soulgraph,
    ) -> Result<Personality, CorruptPersonality> {
        if let Ok(response) = soul
            .patch(format!("/personality/{id}").as_str(), personality)
            .await
        {
            match response.status() {
                reqwest::StatusCode::OK => match response.json::<Personality>().await {
                    Ok(p) => Ok(p),
                    Err(_) => Err(CorruptPersonality),
                },
                _ => Err(CorruptPersonality),
            }
        } else {
            Err(CorruptPersonality)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use traits::TraitBuilder;

    #[test]
    fn test_personality_builder_validation() {
        // Test missing name
        let result = PersonalityBuilder::new()
            .add_trait(TraitBuilder::new("test").build())
            .build();
        assert!(matches!(result, Err(PersonalityBuilderError::MissingName)));

        // Test missing traits
        let result = PersonalityBuilder::new().name("Test").build();
        assert!(matches!(result, Err(PersonalityBuilderError::NoTraits)));
    }

    #[test]
    fn test_personality_builder_minimal() {
        let trait_ = TraitBuilder::new("kind")
            .strength(0.7)
            .add_expression_rule("helps others")
            .build();

        let personality = Personality::builder()
            .name("Test Personality")
            .add_trait(trait_)
            .build()
            .unwrap();

        assert_eq!(personality.name, "Test Personality");
        assert_eq!(personality.traits.len(), 1);
        assert!(personality.metadata.is_some());
        assert!(personality
            .metadata
            .as_ref()
            .unwrap()
            .contains_key("creation_date"));
        assert!(personality
            .metadata
            .as_ref()
            .unwrap()
            .contains_key("last_modified"));
    }

    #[test]
    fn test_personality_builder_complete() {
        let id = uuid::Uuid::new_v4();
        let trait1 = TraitBuilder::new("sarcastic")
            .strength(0.95)
            .add_expression_rule("use irony to highlight obvious mistakes")
            .add_expression_rule("employ mock praise for poor decisions")
            .build();

        let trait2 = TraitBuilder::new("memetic")
            .strength(0.9)
            .add_expression_rule("reference popular trading memes")
            .add_expression_rule("create memorable catchphrases")
            .build();

        let personality = Personality::builder()
            .id(id)
            .name("Dr. Luna")
            .add_trait(trait1)
            .add_trait(trait2)
            .set_metadata("creation_date", "2025-01-11")
            .set_metadata("last_modified", "2025-01-11")
            .build()
            .unwrap();

        assert_eq!(personality.id, Some(id));
        assert_eq!(personality.name, "Dr. Luna");
        assert_eq!(personality.traits.len(), 2);
        assert_eq!(personality.traits[0].r#trait, "sarcastic");
        assert_eq!(personality.traits[1].r#trait, "memetic");

        let metadata = personality.metadata.unwrap();
        assert_eq!(metadata.get("creation_date").unwrap(), "2025-01-11");
        assert_eq!(metadata.get("last_modified").unwrap(), "2025-01-11");
    }

    #[test]
    fn test_default_personality() {
        let personality = Personality::default();

        assert!(personality.id.is_none());
        assert_eq!(personality.name, "AI Assistant");
        assert_eq!(personality.traits.len(), 2);
        assert!(personality.values.is_none());
        assert!(personality.voice.is_none());
        assert!(personality.relationship.is_none());
        assert!(personality.metadata.is_some());

        let metadata = personality.metadata.unwrap();
        assert!(metadata.contains_key("creation_date"));
        assert!(metadata.contains_key("last_modified"));

        let helpful = &personality.traits[0];
        assert!(helpful.id.is_none());
        assert_eq!(helpful.r#trait, "helpful");
        assert_eq!(helpful.strength, 0.9);
        assert_eq!(
            helpful.expression_rules.as_ref().unwrap()[0],
            "always seeks to assist"
        );

        let professional = &personality.traits[1];
        assert!(professional.id.is_none());
        assert_eq!(professional.r#trait, "professional");
        assert_eq!(professional.strength, 0.8);
        assert_eq!(
            professional.expression_rules.as_ref().unwrap()[0],
            "maintains appropriate boundaries"
        );
    }
}
