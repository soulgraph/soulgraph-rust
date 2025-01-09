use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub r#type: MemoryType,
    pub content: String,
    pub timestamp: i64,
    pub importance: f32,
    pub emotional_valence: f32,
    pub context: Context,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryType {
    Observation,
    Reflection,
    Memory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub user_state: String,
    pub agent_state: String,
    pub topic: String,
    pub interaction_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryGraph {
    pub memories: HashMap<String, Memory>,
    pub personality_state: PersonalityState,
    pub indices: Indices,
    pub stats: Stats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalityState {
    pub base_traits: HashMap<String, f32>,
    pub last_evolution: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Indices {
    pub temporal: HashMap<String, serde_json::Value>,
    pub emotional: HashMap<String, serde_json::Value>,
    pub semantic: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub total_memories: i32,
    pub last_consolidation: i64,
}
