use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    pub style: String,
    pub tone: String,
    pub qualities: Vec<String>,
    pub patterns: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_voice_serialization() {
        let voice = Voice {
            style: "casual".to_string(),
            tone: "friendly".to_string(),
            qualities: vec!["warm".to_string(), "engaging".to_string()],
            patterns: vec!["uses metaphors".to_string()],
        };

        let serialized = serde_json::to_value(&voice).unwrap();
        assert_eq!(serialized["style"], "casual");
        assert_eq!(serialized["tone"], "friendly");
        assert_eq!(serialized["qualities"][0], "warm");
    }

    #[test]
    fn test_voice_deserialization() {
        let json = json!({
            "style": "formal",
            "tone": "professional",
            "qualities": ["articulate", "precise"],
            "patterns": ["uses technical terms"]
        });

        let voice: Voice = serde_json::from_value(json).unwrap();
        assert_eq!(voice.style, "formal");
        assert_eq!(voice.qualities[0], "articulate");
    }
}
