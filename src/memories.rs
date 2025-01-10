mod fragment;

use fragment::Fragment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCollection {
    pub memories: HashMap<String, CoreMemory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMemory {
    pub id: String,
    pub core_memory: String,
    pub fragments: Vec<Fragment>,
    pub connections: Vec<String>,
    pub emotional_signature: EmotionalSignature,
    pub importance_score: f32,
    pub creation_date: i64,
    pub last_accessed: i64,
    pub metadata: MemoryMetadata,
}

impl Default for CoreMemory {
    fn default() -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: String::new(),
            core_memory: String::new(),
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
    pub valence: f32,
    pub intensity: f32,
}

impl Default for EmotionalSignature {
    fn default() -> Self {
        Self {
            valence: 0.0,
            intensity: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    pub topic_tags: Vec<String>,
    pub personality_influence: Vec<String>,
    pub memory_type: String,
}

impl Default for MemoryMetadata {
    fn default() -> Self {
        Self {
            topic_tags: Vec::new(),
            personality_influence: Vec::new(),
            memory_type: "default".to_string(),
        }
    }
}

#[derive(Default)]
pub struct CoreMemoryBuilder {
    id: String,
    core_memory: String,
    fragments: Vec<Fragment>,
    connections: Vec<String>,
    emotional_signature: Option<EmotionalSignature>,
    importance_score: f32,
    creation_date: i64,
    last_accessed: Option<i64>,
    metadata: Option<MemoryMetadata>,
}

impl CoreMemoryBuilder {
    pub fn new(id: String, core_memory: String) -> Self {
        Self {
            id,
            core_memory,
            creation_date: chrono::Utc::now().timestamp_millis(),
            ..Default::default()
        }
    }

    pub fn add_fragment(mut self, fragment: Fragment) -> Self {
        self.fragments.push(fragment);
        self
    }

    pub fn add_connection(mut self, connection: String) -> Self {
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

    pub fn build(self) -> CoreMemory {
        CoreMemory {
            id: self.id,
            core_memory: self.core_memory,
            fragments: self.fragments,
            connections: self.connections,
            emotional_signature: self.emotional_signature.unwrap_or(EmotionalSignature {
                valence: 0.0,
                intensity: 0.0,
            }),
            importance_score: self.importance_score,
            creation_date: self.creation_date,
            last_accessed: self
                .last_accessed
                .unwrap_or_else(|| chrono::Utc::now().timestamp_millis()),
            metadata: self.metadata.unwrap_or_else(|| MemoryMetadata {
                topic_tags: Vec::new(),
                personality_influence: Vec::new(),
                memory_type: "default".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_core_memory() {
        let memory = CoreMemory::default();
        assert!(memory.id.is_empty());
        assert!(memory.core_memory.is_empty());
        assert!(memory.fragments.is_empty());
        assert!(memory.connections.is_empty());
        assert_eq!(memory.emotional_signature.valence, 0.0);
        assert_eq!(memory.emotional_signature.intensity, 0.0);
        assert_eq!(memory.importance_score, 0.0);
        assert!(memory.metadata.topic_tags.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let memory = CoreMemoryBuilder::new("test-id".to_string(), "test memory".to_string())
            .add_fragment(Fragment::default())
            .add_connection("connection-1".to_string())
            .emotional_signature(EmotionalSignature {
                valence: 0.5,
                intensity: 0.7,
            })
            .importance_score(0.8)
            .build();

        assert_eq!(memory.id, "test-id");
        assert_eq!(memory.core_memory, "test memory");
        assert_eq!(memory.fragments.len(), 1);
        assert_eq!(memory.connections, vec!["connection-1"]);
        assert_eq!(memory.emotional_signature.valence, 0.5);
        assert_eq!(memory.emotional_signature.intensity, 0.7);
        assert_eq!(memory.importance_score, 0.8);
    }

    #[test]
    fn test_serde() {
        let memory = CoreMemoryBuilder::new("test-id".to_string(), "test memory".to_string())
            .importance_score(0.8)
            .build();

        let serialized = serde_json::to_string(&memory).unwrap();
        let deserialized: CoreMemory = serde_json::from_str(&serialized).unwrap();

        assert_eq!(memory.id, deserialized.id);
        assert_eq!(memory.core_memory, deserialized.core_memory);
        assert_eq!(memory.importance_score, deserialized.importance_score);
    }
}
