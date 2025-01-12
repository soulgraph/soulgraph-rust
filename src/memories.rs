mod fragment;

use fragment::Fragment;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error, fmt};
use uuid::Uuid;

use crate::Soulgraph;

#[derive(Debug)]
pub struct CorruptMemory;

impl fmt::Display for CorruptMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "personality is corrupted")
    }
}

impl error::Error for CorruptMemory {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCollection {
    pub memories: HashMap<Uuid, Memory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub memory: String,
    pub fragments: Vec<Fragment>,
    #[serde(with = "uuid_vec_format")]
    pub connections: Vec<Uuid>,
    pub emotional_signature: EmotionalSignature,
    #[serde(with = "validate_importance_score")]
    pub importance_score: f32,
    pub creation_date: i64,
    pub last_accessed: i64,
    pub metadata: MemoryMetadata,
}

impl Memory {
    /// Creates a new MemoryBuilder instance for constructing a Memory
    pub fn builder() -> MemoryBuilder {
        MemoryBuilder::default()
    }

    /// Get the `Memory` with the given `id`.
    pub async fn get(id: &str, soul: Soulgraph) -> Result<Memory, CorruptMemory> {
        if let Ok(response) = soul.get(format!("/personality/{id}").as_str()).await {
            match response.status() {
                reqwest::StatusCode::OK => match response.json::<Memory>().await {
                    Ok(p) => Ok(p),
                    Err(_) => Err(CorruptMemory),
                },
                _ => Err(CorruptMemory),
            }
        } else {
            Err(CorruptMemory)
        }
    }

    /// Create a `Memory`.
    pub async fn create(personality: &Memory, soul: Soulgraph) -> Result<Memory, CorruptMemory> {
        if let Ok(response) = soul.post("/personality", personality).await {
            match response.status() {
                reqwest::StatusCode::OK => match response.json::<Memory>().await {
                    Ok(p) => Ok(p),
                    Err(_) => Err(CorruptMemory),
                },
                _ => Err(CorruptMemory),
            }
        } else {
            Err(CorruptMemory)
        }
    }

    /// Delete a `Memory` stored under `id`.
    pub async fn delete(id: &str, soul: Soulgraph) -> Result<(), CorruptMemory> {
        if let Ok(response) = soul.delete(format!("/personality/{id}").as_str()).await {
            match response.status() {
                reqwest::StatusCode::OK => Ok(()),
                _ => Err(CorruptMemory),
            }
        } else {
            Err(CorruptMemory)
        }
    }
}

mod uuid_vec_format {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(uuids: &[Uuid], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let strings: Vec<String> = uuids.iter().map(|u| u.to_string()).collect();
        strings.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Uuid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let strings: Vec<String> = Vec::deserialize(deserializer)?;
        strings
            .into_iter()
            .map(|s| Uuid::parse_str(&s).map_err(serde::de::Error::custom))
            .collect()
    }
}

mod validate_importance_score {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(score: &f32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(*score)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let score = f32::deserialize(deserializer)?;
        if !(0.0..=1.0).contains(&score) {
            return Err(serde::de::Error::custom(
                "importance_score must be between 0 and 1",
            ));
        }
        Ok(score)
    }
}

impl Default for Memory {
    fn default() -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: None,
            memory: String::new(),
            fragments: Vec::new(),
            connections: Vec::new(),
            emotional_signature: EmotionalSignature::default(),
            importance_score: 0.0,
            creation_date: now,
            last_accessed: now,
            metadata: MemoryMetadata::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalSignature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(with = "validate_valence")]
    pub valence: f32,
    #[serde(with = "validate_intensity")]
    pub intensity: f32,
}

impl Default for EmotionalSignature {
    fn default() -> Self {
        Self {
            id: None,
            valence: 0.0,
            intensity: 0.0,
        }
    }
}

mod validate_valence {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(valence: &f32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(*valence)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let valence = f32::deserialize(deserializer)?;
        if !(-1.0..=1.0).contains(&valence) {
            return Err(serde::de::Error::custom("valence must be between -1 and 1"));
        }
        Ok(valence)
    }
}

mod validate_intensity {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(intensity: &f32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(*intensity)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intensity = f32::deserialize(deserializer)?;
        if !(0.0..=1.0).contains(&intensity) {
            return Err(serde::de::Error::custom(
                "intensity must be between 0 and 1",
            ));
        }
        Ok(intensity)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub topic_tags: Vec<String>,
    pub personality_influence: Vec<String>,
    pub memory_type: String,
}

impl Default for MemoryMetadata {
    fn default() -> Self {
        Self {
            id: None,
            topic_tags: Vec::new(),
            personality_influence: Vec::new(),
            memory_type: "default".to_string(),
        }
    }
}

#[derive(Default)]
pub struct MemoryBuilder {
    memory: String,
    fragments: Vec<Fragment>,
    connections: Vec<Uuid>,
    emotional_signature: Option<EmotionalSignature>,
    importance_score: f32,
    creation_date: i64,
    last_accessed: Option<i64>,
    metadata: Option<MemoryMetadata>,
}

impl MemoryBuilder {
    pub fn new(memory: String) -> Self {
        Self {
            memory,
            creation_date: chrono::Utc::now().timestamp_millis(),
            ..Default::default()
        }
    }

    pub fn add_fragment(mut self, fragment: Fragment) -> Self {
        self.fragments.push(fragment);
        self
    }

    pub fn add_connection(mut self, connection: Uuid) -> Self {
        self.connections.push(connection);
        self
    }

    pub fn emotional_signature(mut self, signature: EmotionalSignature) -> Self {
        self.emotional_signature = Some(signature);
        self
    }

    pub fn importance_score(mut self, score: f32) -> Self {
        self.importance_score = score;
        self
    }

    pub fn last_accessed(mut self, timestamp: i64) -> Self {
        self.last_accessed = Some(timestamp);
        self
    }

    pub fn metadata(mut self, metadata: MemoryMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Memory {
        Memory {
            id: None,
            memory: self.memory,
            fragments: self.fragments,
            connections: self.connections,
            emotional_signature: self.emotional_signature.unwrap_or_default(),
            importance_score: self.importance_score,
            creation_date: self.creation_date,
            last_accessed: self
                .last_accessed
                .unwrap_or_else(|| chrono::Utc::now().timestamp_millis()),
            metadata: self.metadata.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_core_memory() {
        let memory = Memory::default();
        assert!(memory.id.is_none());
        assert!(memory.memory.is_empty());
        assert!(memory.fragments.is_empty());
        assert!(memory.connections.is_empty());
        assert_eq!(memory.emotional_signature.valence, 0.0);
        assert_eq!(memory.emotional_signature.intensity, 0.0);
        assert_eq!(memory.importance_score, 0.0);
        assert!(memory.metadata.topic_tags.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let uuid = Uuid::new_v4();
        let memory = MemoryBuilder::new("test memory".to_string())
            .add_fragment(Fragment::default())
            .add_connection(uuid)
            .emotional_signature(EmotionalSignature {
                id: None,
                valence: 0.5,
                intensity: 0.7,
            })
            .importance_score(0.8)
            .build();

        assert!(memory.id.is_none());
        assert_eq!(memory.memory, "test memory");
        assert_eq!(memory.fragments.len(), 1);
        assert_eq!(memory.connections, vec![uuid]);
        assert_eq!(memory.emotional_signature.valence, 0.5);
        assert_eq!(memory.emotional_signature.intensity, 0.7);
        assert_eq!(memory.importance_score, 0.8);
    }

    #[test]
    fn test_serde() {
        let memory = MemoryBuilder::new("test memory".to_string())
            .importance_score(0.8)
            .build();

        let serialized = serde_json::to_string(&memory).unwrap();
        let deserialized: Memory = serde_json::from_str(&serialized).unwrap();

        assert_eq!(memory.id, deserialized.id);
        assert_eq!(memory.memory, deserialized.memory);
        assert_eq!(memory.importance_score, deserialized.importance_score);
    }
}
