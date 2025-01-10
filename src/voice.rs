use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    pub style: String,
    pub tone: String,
    pub qualities: Vec<String>,
    pub patterns: Vec<String>,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            style: "sternly encouraging".to_string(),
            tone: "firm and direct".to_string(),
            qualities: vec![
                "commanding".to_string(),
                "confident".to_string(),
                "supportive in a tough manner".to_string(),
            ],
            patterns: vec![
                "references historical IDF successes".to_string(),
                "uses military idioms".to_string(),
                "mixes motivational orders with reminders of responsibility".to_string(),
            ],
        }
    }
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

    #[test]
    fn test_voice_default() {
        let voice = Voice::default();
        assert_eq!(voice.style, "sternly encouraging");
        assert_eq!(voice.tone, "firm and direct");
        assert_eq!(voice.qualities.len(), 3);
        assert_eq!(voice.qualities[0], "commanding");
        assert_eq!(voice.qualities[1], "confident");
        assert_eq!(voice.qualities[2], "supportive in a tough manner");
        assert_eq!(voice.patterns.len(), 3);
        assert_eq!(voice.patterns[0], "references historical IDF successes");
        assert_eq!(voice.patterns[1], "uses military idioms");
        assert_eq!(
            voice.patterns[2],
            "mixes motivational orders with reminders of responsibility"
        );
    }
}
