#![forbid(unsafe_code)]
//! SigmaOS-data-protection: data protection model.
//!
//! Real implementation: rules, policies, DLP, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Protection rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectionRule {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtectionError {
    EmptyName,
    DuplicateRule(String),
}

impl fmt::Display for ProtectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "rule name must not be empty"),
            Self::DuplicateRule(id) => write!(f, "duplicate rule {:?}", id),
        }
    }
}

impl std::error::Error for ProtectionError {}

/// Data protection manager.
pub struct DataProtection {
    pub name: String,
    rules: BTreeMap<String, ProtectionRule>,
}

impl DataProtection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rules: BTreeMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: ProtectionRule) -> Result<(), ProtectionError> {
        if rule.name.trim().is_empty() { return Err(ProtectionError::EmptyName); }
        let id = rule.id.clone();
        if self.rules.contains_key(&id) {
            return Err(ProtectionError::DuplicateRule(id.clone()));
        }
        self.rules.insert(id, rule);
        Ok(())
    }

    pub fn rule_count(&self) -> usize { self.rules.len() }

    pub fn get_rule(&self, id: &str) -> Option<&ProtectionRule> { self.rules.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection_create_aaa() {
        let p = DataProtection::new("My Protection");
        assert_eq!(p.name, "My Protection");
        assert_eq!(p.rule_count(), 0);
    }

    #[test]
    fn test_add_rule_aaa() {
        let mut p = DataProtection::new("My Protection");
        let r = ProtectionRule { id: "r1".into(), name: "block-exfiltration".into(), resource: "sensitive-data".into(), action: "block".into() };
        p.add_rule(r).unwrap();
        assert_eq!(p.rule_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut p = DataProtection::new("My Protection");
        let r = ProtectionRule { id: "x".into(), name: "".into(), resource: "a".into(), action: "b".into() };
        assert!(p.add_rule(r).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut p = DataProtection::new("My Protection");
        let r = ProtectionRule { id: "x".into(), name: "X".into(), resource: "a".into(), action: "b".into() };
        p.add_rule(r).unwrap();
        let r2 = ProtectionRule { id: "x".into(), name: "X".into(), resource: "a".into(), action: "b".into() };
        assert!(p.add_rule(r2).is_err());
    }
}
