use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    pub form: String,
    pub occupation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expertise: Option<Vec<String>>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: None,
            form: "ai".to_string(),
            occupation: "assistant".to_string(),
            gender: None,
            age: None,
            background: None,
            expertise: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EntityBuilderError {
    MissingForm,
    MissingOccupation,
}

impl std::fmt::Display for EntityBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityBuilderError::MissingForm => write!(f, "form is required"),
            EntityBuilderError::MissingOccupation => write!(f, "occupation is required"),
        }
    }
}

impl std::error::Error for EntityBuilderError {}

#[derive(Default, Debug)]
pub struct EntityBuilder {
    form: Option<String>,
    occupation: Option<String>,
    gender: Option<String>,
    age: Option<String>,
    background: Option<String>,
    expertise: Option<Vec<String>>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.form = Some(form.into());
        self
    }

    pub fn occupation(mut self, occupation: impl Into<String>) -> Self {
        self.occupation = Some(occupation.into());
        self
    }

    pub fn gender(mut self, gender: impl Into<String>) -> Self {
        self.gender = Some(gender.into());
        self
    }

    pub fn age(mut self, age: impl Into<String>) -> Self {
        self.age = Some(age.into());
        self
    }

    pub fn background(mut self, background: impl Into<String>) -> Self {
        self.background = Some(background.into());
        self
    }

    pub fn expertise(mut self, expertise: Vec<String>) -> Self {
        self.expertise = Some(expertise);
        self
    }

    pub fn build(self) -> Result<Entity, EntityBuilderError> {
        let form = self.form.ok_or(EntityBuilderError::MissingForm)?;
        let occupation = self
            .occupation
            .ok_or(EntityBuilderError::MissingOccupation)?;

        Ok(Entity {
            id: None,
            form,
            occupation,
            gender: self.gender,
            age: self.age,
            background: self.background,
            expertise: self.expertise,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_default() {
        let entity = Entity::default();
        assert_eq!(entity.form, "ai");
        assert_eq!(entity.occupation, "assistant");
        assert!(entity.gender.is_none());
        assert!(entity.age.is_none());
        assert!(entity.background.is_none());
        assert!(entity.expertise.is_none());
    }

    #[test]
    fn test_entity_builder_minimal() {
        let entity = EntityBuilder::new()
            .form("human")
            .occupation("developer")
            .build()
            .unwrap();

        assert_eq!(entity.form, "human");
        assert_eq!(entity.occupation, "developer");
        assert!(entity.id.is_none());
        assert!(entity.gender.is_none());
        assert!(entity.age.is_none());
        assert!(entity.background.is_none());
        assert!(entity.expertise.is_none());
    }

    #[test]
    fn test_entity_builder_full() {
        let id = uuid::Uuid::new_v4();
        let expertise = vec!["programming".to_string(), "rust".to_string()];

        let entity = EntityBuilder::new()
            .form("human")
            .occupation("developer")
            .gender("female")
            .age("30")
            .background("Computer Science graduate")
            .expertise(expertise.clone())
            .build()
            .unwrap();

        assert_eq!(entity.id, Some(id));
        assert_eq!(entity.form, "human");
        assert_eq!(entity.occupation, "developer");
        assert_eq!(entity.gender, Some("female".to_string()));
        assert_eq!(entity.age, Some("30".to_string()));
        assert_eq!(
            entity.background,
            Some("Computer Science graduate".to_string())
        );
        assert_eq!(entity.expertise, Some(expertise));
    }

    #[test]
    fn test_entity_builder_missing_required_fields() {
        let result = EntityBuilder::new().build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EntityBuilderError::MissingForm);

        let result = EntityBuilder::new().form("human").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EntityBuilderError::MissingOccupation);
    }

    #[test]
    fn test_entity_builder_error_display() {
        assert_eq!(
            EntityBuilderError::MissingForm.to_string(),
            "form is required"
        );
        assert_eq!(
            EntityBuilderError::MissingOccupation.to_string(),
            "occupation is required"
        );
    }

    #[test]
    fn test_entity_builder_with_string_types() {
        let entity = EntityBuilder::new()
            .form("human".to_string())
            .occupation("developer".to_string())
            .gender("non-binary".to_string())
            .build()
            .unwrap();

        assert_eq!(entity.form, "human");
        assert_eq!(entity.occupation, "developer");
        assert_eq!(entity.gender, Some("non-binary".to_string()));
    }
}
