use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error, fmt};

use crate::Soulgraph;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Personality {
    pub core_traits: Vec<Trait>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            core_traits: vec![
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Trait {
    pub r#trait: String,
    pub strength: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_rules: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct CorruptPersonality;

impl fmt::Display for CorruptPersonality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "personality is corrupted")
    }
}

impl error::Error for CorruptPersonality {}

impl Personality {
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
