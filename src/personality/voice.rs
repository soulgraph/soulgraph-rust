use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub style: String,
    pub tone: String,
    pub qualities: Vec<String>,
    pub patterns: Vec<String>,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            id: None,
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

impl Voice {
    pub fn builder() -> VoiceBuilder {
        VoiceBuilder::default()
    }
}

#[derive(Default)]
pub struct VoiceBuilder {
    style: Option<String>,
    tone: Option<String>,
    qualities: Option<Vec<String>>,
    patterns: Option<Vec<String>>,
}

impl VoiceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    pub fn tone(mut self, tone: impl Into<String>) -> Self {
        self.tone = Some(tone.into());
        self
    }

    pub fn qualities(mut self, qualities: Vec<String>) -> Self {
        self.qualities = Some(qualities);
        self
    }

    pub fn patterns(mut self, patterns: Vec<String>) -> Self {
        self.patterns = Some(patterns);
        self
    }

    pub fn build(self) -> Result<Voice, &'static str> {
        Ok(Voice {
            id: None,
            style: self.style.ok_or("style is required")?,
            tone: self.tone.ok_or("tone is required")?,
            qualities: self.qualities.ok_or("qualities is required")?,
            patterns: self.patterns.ok_or("patterns is required")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_voice_serialization() {
        let voice = Voice {
            id: None,
            style: "casual".to_string(),
            tone: "friendly".to_string(),
            qualities: vec!["warm".to_string(), "engaging".to_string()],
            patterns: vec!["uses metaphors".to_string()],
        };

        let serialized = serde_json::to_value(&voice).unwrap();
        let obj = serialized.as_object().unwrap();
        assert_eq!(obj["style"], "casual");
        assert_eq!(obj["tone"], "friendly");
        assert_eq!(obj["qualities"][0], "warm");
        assert!(!obj.contains_key("id"));
    }

    #[test]
    fn test_voice_with_id_serialization() {
        let id = Uuid::new_v4();
        let voice = Voice {
            id: Some(id),
            style: "casual".to_string(),
            tone: "friendly".to_string(),
            qualities: vec!["warm".to_string()],
            patterns: vec!["uses metaphors".to_string()],
        };

        let serialized = serde_json::to_value(&voice).unwrap();
        assert_eq!(serialized["id"], id.to_string());
    }

    #[test]
    fn test_voice_deserialization() {
        let id = Uuid::new_v4();
        let json = json!({
            "id": id.to_string(),
            "style": "formal",
            "tone": "professional",
            "qualities": ["articulate", "precise"],
            "patterns": ["uses technical terms"]
        });

        let voice: Voice = serde_json::from_value(json).unwrap();
        assert_eq!(voice.id, Some(id));
        assert_eq!(voice.style, "formal");
        assert_eq!(voice.qualities[0], "articulate");
    }

    #[test]
    fn test_voice_builder() {
        let voice = VoiceBuilder::new()
            .style("casual")
            .tone("friendly")
            .qualities(vec!["warm".to_string(), "engaging".to_string()])
            .patterns(vec!["uses metaphors".to_string()])
            .build()
            .unwrap();

        assert_eq!(voice.style, "casual");
        assert_eq!(voice.tone, "friendly");
        assert_eq!(voice.qualities[0], "warm");
        assert!(voice.id.is_none());
    }

    #[test]
    fn test_voice_builder_validation() {
        let result = VoiceBuilder::new().build();
        assert!(result.is_err());

        let result = VoiceBuilder::new().style("casual").build();
        assert!(result.is_err());

        let result = VoiceBuilder::new().style("casual").tone("friendly").build();
        assert!(result.is_err());

        let result = VoiceBuilder::new()
            .style("casual")
            .tone("friendly")
            .qualities(vec!["warm".to_string()])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_voice_default() {
        let voice = Voice::default();
        assert!(voice.id.is_none());
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
