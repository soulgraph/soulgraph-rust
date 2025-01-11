mod traits;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error, fmt};
use traits::Trait;

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
    pub traits: Vec<Trait>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            traits: vec![
                Trait {
                    r#trait: "helpful".to_string(),
                    strength: 0.9,
                    expression_rules: Some(vec!["always seeks to assist".to_string()]),
                },
                Trait {
                    r#trait: "professional".to_string(),
                    strength: 0.8,
                    expression_rules: Some(vec!["maintains appropriate boundaries".to_string()]),
                },
            ],
            metadata: None,
        }
    }
}

#[derive(Default)]
pub struct PersonalityBuilder {
    traits: Vec<Trait>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PersonalityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_trait(mut self, trait_: Trait) -> Self {
        self.traits.push(trait_);
        self
    }

    pub fn set_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        if let Some(metadata) = self.metadata.as_mut() {
            metadata.insert(key.to_string(), value);
        }
        self
    }

    pub fn build(self) -> Personality {
        Personality {
            traits: self.traits,
            metadata: self.metadata,
        }
    }
}

impl Personality {
    pub fn new() -> Self {
        Self {
            traits: Vec::new(),
            metadata: None,
        }
    }

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
    fn test_personality_builder_empty() {
        let personality = Personality::builder().build();

        assert!(personality.traits.is_empty());
        assert!(personality.metadata.is_none());
    }

    #[test]
    fn test_personality_builder_single_trait() {
        let trait_ = TraitBuilder::new("kind")
            .strength(0.7)
            .add_expression_rule("helps others")
            .build();

        let personality = Personality::builder().add_trait(trait_).build();

        assert_eq!(personality.traits.len(), 1);
        let added_trait = &personality.traits[0];
        assert_eq!(added_trait.r#trait, "kind");
        assert_eq!(added_trait.strength, 0.7);
        assert_eq!(
            added_trait.expression_rules.as_ref().unwrap()[0],
            "helps others"
        );
    }

    #[test]
    fn test_personality_builder_metadata() {
        let personality = Personality::builder()
            .set_metadata("version", serde_json::Value::String("1.0".to_string()))
            .set_metadata("author", serde_json::Value::String("test".to_string()))
            .build();

        assert!(personality.metadata.is_some());
        let metadata = personality.metadata.unwrap();
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata.get("version").unwrap(), "1.0");
        assert_eq!(metadata.get("author").unwrap(), "test");
    }

    #[test]
    fn test_personality_builder_complex() {
        let trait1 = TraitBuilder::new("intelligent")
            .strength(0.9)
            .add_expression_rule("solves problems")
            .build();

        let trait2 = TraitBuilder::new("creative")
            .strength(0.8)
            .add_expression_rule("thinks outside the box")
            .build();

        let personality = Personality::builder()
            .add_trait(trait1)
            .add_trait(trait2)
            .set_metadata("created_at", serde_json::Value::String("2024".to_string()))
            .set_metadata(
                "version",
                serde_json::Value::Number(serde_json::Number::from(1)),
            )
            .build();

        // Test traits
        assert_eq!(personality.traits.len(), 2);
        assert_eq!(personality.traits[0].r#trait, "intelligent");
        assert_eq!(personality.traits[1].r#trait, "creative");

        // Test metadata
        let metadata = personality.metadata.unwrap();
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata.get("created_at").unwrap(), "2024");
        assert_eq!(metadata.get("version").unwrap().as_i64().unwrap(), 1);
    }

    #[test]
    fn test_personality_builder_method_chaining_order() {
        let trait_ = TraitBuilder::new("adaptable").build();

        let personality1 = Personality::builder()
            .add_trait(trait_.clone())
            .set_metadata("key", serde_json::Value::String("value".to_string()))
            .build();

        let personality2 = Personality::builder()
            .set_metadata("key", serde_json::Value::String("value".to_string()))
            .add_trait(trait_)
            .build();

        assert_eq!(personality1.traits, personality2.traits);
        assert_eq!(personality1.metadata, personality2.metadata);
    }

    #[test]
    fn test_default_personality() {
        let personality = Personality::default();

        assert_eq!(personality.traits.len(), 2);
        assert!(personality.metadata.is_none());

        let helpful = &personality.traits[0];
        assert_eq!(helpful.r#trait, "helpful");
        assert_eq!(helpful.strength, 0.9);
        assert_eq!(
            helpful.expression_rules.as_ref().unwrap()[0],
            "always seeks to assist"
        );

        let professional = &personality.traits[1];
        assert_eq!(professional.r#trait, "professional");
        assert_eq!(professional.strength, 0.8);
        assert_eq!(
            professional.expression_rules.as_ref().unwrap()[0],
            "maintains appropriate boundaries"
        );
    }
}
