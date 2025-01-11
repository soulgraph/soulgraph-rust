use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Trait {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub r#trait: String,
    pub strength: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_rules: Option<Vec<String>>,
}

#[derive(Default)]
pub struct TraitBuilder {
    trait_name: String,
    strength: f32,
    expression_rules: Option<Vec<String>>,
}

impl TraitBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            trait_name: name.to_string(),
            strength: 0.5, // default strength
            expression_rules: None,
        }
    }

    pub fn strength(mut self, strength: f32) -> Self {
        self.strength = strength;
        self
    }

    pub fn add_expression_rule(mut self, rule: &str) -> Self {
        if self.expression_rules.is_none() {
            self.expression_rules = Some(Vec::new());
        }
        if let Some(rules) = self.expression_rules.as_mut() {
            rules.push(rule.to_string());
        }
        self
    }

    pub fn build(self) -> Trait {
        Trait {
            id: None,
            r#trait: self.trait_name,
            strength: self.strength,
            expression_rules: self.expression_rules,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_builder_default_values() {
        let trait_ = TraitBuilder::new("friendly").build();

        assert_eq!(trait_.r#trait, "friendly");
        assert_eq!(trait_.strength, 0.5); // Check default strength
        assert!(trait_.expression_rules.is_none());
    }

    #[test]
    fn test_trait_builder_with_strength() {
        let trait_ = TraitBuilder::new("intelligent").strength(0.8).build();

        assert_eq!(trait_.r#trait, "intelligent");
        assert_eq!(trait_.strength, 0.8);
        assert!(trait_.expression_rules.is_none());
    }

    #[test]
    fn test_trait_builder_with_single_rule() {
        let trait_ = TraitBuilder::new("helpful")
            .add_expression_rule("always offers assistance")
            .build();

        assert_eq!(trait_.r#trait, "helpful");
        assert_eq!(trait_.strength, 0.5);
        assert_eq!(
            trait_.expression_rules.unwrap(),
            vec!["always offers assistance"]
        );
    }

    #[test]
    fn test_trait_builder_with_multiple_rules() {
        let trait_ = TraitBuilder::new("creative")
            .add_expression_rule("thinks outside the box")
            .add_expression_rule("provides unique solutions")
            .add_expression_rule("explores new possibilities")
            .build();

        let rules = trait_.expression_rules.unwrap();
        assert_eq!(rules.len(), 3);
        assert!(rules.contains(&"thinks outside the box".to_string()));
        assert!(rules.contains(&"provides unique solutions".to_string()));
        assert!(rules.contains(&"explores new possibilities".to_string()));
    }

    #[test]
    fn test_trait_builder_full_configuration() {
        let trait_ = TraitBuilder::new("empathetic")
            .strength(0.9)
            .add_expression_rule("listens carefully")
            .add_expression_rule("shows understanding")
            .build();

        assert_eq!(trait_.r#trait, "empathetic");
        assert_eq!(trait_.strength, 0.9);

        let rules = trait_.expression_rules.unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0], "listens carefully");
        assert_eq!(rules[1], "shows understanding");
    }

    #[test]
    fn test_trait_builder_method_chaining_order() {
        let trait1 = TraitBuilder::new("adaptable")
            .strength(0.7)
            .add_expression_rule("rule1")
            .build();

        let trait2 = TraitBuilder::new("adaptable")
            .add_expression_rule("rule1")
            .strength(0.7)
            .build();

        assert_eq!(trait1, trait2);
    }
}
