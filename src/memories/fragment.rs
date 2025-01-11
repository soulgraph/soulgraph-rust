use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FragmentType {
    #[default]
    Observation,
    Reflection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub id: Option<Uuid>,
    pub topic: String,
    pub user_state: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            id: None,
            topic: "general".to_string(),
            user_state: "neutral".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    pub id: Option<Uuid>,
    #[serde(rename = "type")]
    pub fragment_type: FragmentType,
    pub content: String,
    pub timestamp: i64,
    #[serde(with = "validate_importance")]
    pub importance: f32,
    #[serde(with = "validate_emotional_valence")]
    pub emotional_valence: f32,
    pub context: Context,
}

mod validate_importance {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(importance: &f32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(*importance)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let importance = f32::deserialize(deserializer)?;
        if !(0.0..=1.0).contains(&importance) {
            return Err(serde::de::Error::custom(
                "importance must be between 0 and 1",
            ));
        }
        Ok(importance)
    }
}

mod validate_emotional_valence {
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
            return Err(serde::de::Error::custom(
                "emotional_valence must be between -1 and 1",
            ));
        }
        Ok(valence)
    }
}

#[derive(Default)]
pub struct FragmentBuilder {
    uuid: Option<Uuid>,
    fragment_type: FragmentType,
    content: String,
    timestamp: Option<i64>,
    importance: f32,
    emotional_valence: f32,
    context: Option<Context>,
}

impl Default for Fragment {
    fn default() -> Self {
        Self {
            id: None,
            fragment_type: FragmentType::default(),
            content: String::new(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            importance: 0.0,
            emotional_valence: 0.0,
            context: Context::default(),
        }
    }
}

impl Fragment {
    pub fn builder() -> FragmentBuilder {
        FragmentBuilder::default()
    }
}
impl FragmentBuilder {
    pub fn new(fragment_type: FragmentType, content: String) -> Self {
        Self {
            fragment_type,
            content,
            ..Default::default()
        }
    }
    pub fn fragment_type(mut self, fragment_type: FragmentType) -> Self {
        self.fragment_type = fragment_type;
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn importance(mut self, importance: f32) -> Self {
        self.importance = importance;
        self
    }

    pub fn emotional_valence(mut self, valence: f32) -> Self {
        self.emotional_valence = valence;
        self
    }

    pub fn context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }

    pub fn build(self) -> Fragment {
        Fragment {
            id: None,
            fragment_type: self.fragment_type,
            content: self.content,
            timestamp: self
                .timestamp
                .unwrap_or_else(|| chrono::Utc::now().timestamp_millis()),
            importance: self.importance,
            emotional_valence: self.emotional_valence,
            context: self.context.unwrap_or_else(|| Context {
                id: None,
                topic: "general".to_string(),
                user_state: "neutral".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_fragment_type_default() {
        assert!(matches!(FragmentType::default(), FragmentType::Observation));
    }

    #[test]
    fn test_context_default() {
        let context = Context::default();
        assert_eq!(context.topic, "general");
        assert_eq!(context.user_state, "neutral");
    }

    #[test]
    fn test_fragment_default() {
        let fragment = Fragment::default();
        assert!(matches!(fragment.fragment_type, FragmentType::Observation));
        assert!(fragment.content.is_empty());
        assert!(fragment.timestamp > 0);
        assert_eq!(fragment.importance, 0.0);
        assert_eq!(fragment.emotional_valence, 0.0);
        assert_eq!(fragment.context.topic, "general");
        assert_eq!(fragment.context.user_state, "neutral");
    }

    #[test]
    fn test_fragment_builder() {
        let fragment = FragmentBuilder::new(FragmentType::Reflection, "Test content".to_string())
            .importance(0.8)
            .emotional_valence(0.5)
            .context(Context {
                id: None,
                topic: "test".to_string(),
                user_state: "happy".to_string(),
            })
            .build();

        assert!(matches!(fragment.fragment_type, FragmentType::Reflection));
        assert_eq!(fragment.content, "Test content");
        assert!(fragment.timestamp > 0);
        assert_eq!(fragment.importance, 0.8);
        assert_eq!(fragment.emotional_valence, 0.5);
        assert_eq!(fragment.context.topic, "test");
        assert_eq!(fragment.context.user_state, "happy");
    }

    #[test]
    fn test_fragment_serialization() {
        let fragment =
            FragmentBuilder::new(FragmentType::Observation, "Test observation".to_string())
                .importance(0.7)
                .build();

        let serialized = serde_json::to_string(&fragment).unwrap();
        let deserialized: Fragment = serde_json::from_str(&serialized).unwrap();

        assert!(matches!(
            deserialized.fragment_type,
            FragmentType::Observation
        ));
        assert_eq!(deserialized.content, "Test observation");
        assert_eq!(deserialized.importance, 0.7);
    }

    #[test]
    fn test_fragment_type_serde() {
        let json_obs = json!("observation");
        let json_ref = json!("reflection");

        let obs: FragmentType = serde_json::from_value(json_obs).unwrap();
        let ref_type: FragmentType = serde_json::from_value(json_ref).unwrap();

        assert!(matches!(obs, FragmentType::Observation));
        assert!(matches!(ref_type, FragmentType::Reflection));

        let serialized_obs = serde_json::to_string(&FragmentType::Observation).unwrap();
        let serialized_ref = serde_json::to_string(&FragmentType::Reflection).unwrap();

        assert_eq!(serialized_obs, "\"observation\"");
        assert_eq!(serialized_ref, "\"reflection\"");
    }

    #[test]
    fn test_builder_default() {
        let builder = FragmentBuilder::default();
        let fragment = builder
            .fragment_type(FragmentType::Observation)
            .content("Default test".to_string())
            .build();

        assert!(matches!(fragment.fragment_type, FragmentType::Observation));
        assert_eq!(fragment.content, "Default test");
        assert_eq!(fragment.importance, 0.0);
        assert_eq!(fragment.emotional_valence, 0.0);
        assert_eq!(fragment.context.topic, "general");
    }
}
