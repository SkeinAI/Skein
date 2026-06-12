use std::collections::HashSet;

/// Represents a single permission rule parsed from a configuration string.
#[derive(Debug, Clone, PartialEq)]
pub enum PermissionRule {
    /// Exact match: the skill name must equal the stored string.
    Exact(String),
    /// Prefix match: the skill name must start with the stored string (without the trailing `*`).
    Prefix(String),
}

impl PermissionRule {
    /// Parse a permission rule string.
    /// If the string ends with `*`, it becomes a `Prefix` rule (the `*` is stripped).
    /// Otherwise, it becomes an `Exact` rule.
    pub fn parse(s: &str) -> Self {
        if s.ends_with('*') {
            PermissionRule::Prefix(s[..s.len() - 1].to_string())
        } else {
            PermissionRule::Exact(s.to_string())
        }
    }

    /// Check whether a given skill name matches this rule.
    pub fn matches(&self, skill_name: &str) -> bool {
        match self {
            PermissionRule::Exact(exact) => skill_name == exact,
            PermissionRule::Prefix(prefix) => skill_name.starts_with(prefix),
        }
    }
}

/// Configuration for skill permissions.
#[derive(Debug, Clone, Default)]
pub struct SkillPermissionConfig {
    /// Skills that are explicitly denied.
    pub deny: Vec<PermissionRule>,
    /// Skills that are explicitly allowed.
    pub allow: Vec<PermissionRule>,
    /// Properties that are considered safe to access automatically.
    pub safe_properties: HashSet<String>,
    /// Skills that are auto-approved (no user interaction needed).
    pub auto_approve: Vec<PermissionRule>,
}

/// The result of a permission check.
#[derive(Debug, Clone, PartialEq)]
pub enum PermissionResult {
    /// The skill is denied.
    Denied,
    /// The skill is allowed.
    Allowed,
    /// The skill requires user approval (ask).
    Ask,
}

/// Checks whether a skill is allowed to run based on the permission configuration.
pub struct SkillPermissionChecker {
    config: SkillPermissionConfig,
}

impl SkillPermissionChecker {
    /// Create a new checker with the given configuration.
    pub fn new(config: SkillPermissionConfig) -> Self {
        SkillPermissionChecker { config }
    }

    /// Check if a skill with the given name and requested properties is allowed to run.
    ///
    /// The decision chain is:
    /// 1. If any deny rule matches → Denied
    /// 2. If any allow rule matches → Allowed
    /// 3. If all requested properties are safe → Allowed
    /// 4. If any auto_approve rule matches → Allowed
    /// 5. Otherwise → Ask
    pub fn check(&self, skill_name: &str, requested_properties: &[String]) -> PermissionResult {
        // Step 1: Deny rules take precedence
        for rule in &self.config.deny {
            if rule.matches(skill_name) {
                return PermissionResult::Denied;
            }
        }

        // Step 2: Allow rules
        for rule in &self.config.allow {
            if rule.matches(skill_name) {
                return PermissionResult::Allowed;
            }
        }

        // Step 3: Safe properties
        if requested_properties
            .iter()
            .all(|prop| self.config.safe_properties.contains(prop))
        {
            return PermissionResult::Allowed;
        }

        // Step 4: Auto-approve rules
        for rule in &self.config.auto_approve {
            if rule.matches(skill_name) {
                return PermissionResult::Allowed;
            }
        }

        // Step 5: Fall through to ask
        PermissionResult::Ask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // PermissionRule::parse
    // ------------------------------------------------------------------------

    #[test]
    fn test_parse_exact() {
        let rule = PermissionRule::parse("read_file");
        assert_eq!(rule, PermissionRule::Exact("read_file".to_string()));
    }

    #[test]
    fn test_parse_prefix() {
        let rule = PermissionRule::parse("read_*");
        assert_eq!(rule, PermissionRule::Prefix("read_".to_string()));
    }

    #[test]
    fn test_parse_prefix_without_star() {
        // A string without '*' should be exact, even if it looks like a prefix pattern
        let rule = PermissionRule::parse("read_");
        assert_eq!(rule, PermissionRule::Exact("read_".to_string()));
    }

    #[test]
    fn test_parse_empty_string() {
        let rule = PermissionRule::parse("");
        assert_eq!(rule, PermissionRule::Exact("".to_string()));
    }

    #[test]
    fn test_parse_star_only() {
        let rule = PermissionRule::parse("*");
        assert_eq!(rule, PermissionRule::Prefix("".to_string()));
    }

    // ------------------------------------------------------------------------
    // PermissionRule::matches
    // ------------------------------------------------------------------------

    #[test]
    fn test_exact_match() {
        let rule = PermissionRule::Exact("read_file".to_string());
        assert!(rule.matches("read_file"));
    }

    #[test]
    fn test_exact_no_match() {
        let rule = PermissionRule::Exact("read_file".to_string());
        assert!(!rule.matches("read_file_extra"));
        assert!(!rule.matches("write_file"));
    }

    #[test]
    fn test_prefix_match() {
        let rule = PermissionRule::Prefix("read_".to_string());
        assert!(rule.matches("read_file"));
        assert!(rule.matches("read_data"));
        assert!(rule.matches("read_"));
    }

    #[test]
    fn test_prefix_no_match() {
        let rule = PermissionRule::Prefix("read_".to_string());
        assert!(!rule.matches("write_file"));
        assert!(!rule.matches("reader"));
        assert!(!rule.matches(""));
    }

    #[test]
    fn test_prefix_empty_prefix() {
        // A prefix of "" matches everything
        let rule = PermissionRule::Prefix("".to_string());
        assert!(rule.matches("anything"));
        assert!(rule.matches(""));
    }

    // ------------------------------------------------------------------------
    // SkillPermissionChecker::check – decision chain
    // ------------------------------------------------------------------------

    /// Helper to build a checker with specific rules.
    fn make_checker(
        deny: Vec<&str>,
        allow: Vec<&str>,
        safe_properties: Vec<&str>,
        auto_approve: Vec<&str>,
    ) -> SkillPermissionChecker {
        let config = SkillPermissionConfig {
            deny: deny.into_iter().map(PermissionRule::parse).collect(),
            allow: allow.into_iter().map(PermissionRule::parse).collect(),
            safe_properties: safe_properties.into_iter().map(String::from).collect(),
            auto_approve: auto_approve.into_iter().map(PermissionRule::parse).collect(),
        };
        SkillPermissionChecker::new(config)
    }

    #[test]
    fn test_deny_takes_precedence_over_allow() {
        // Even if a skill is in allow, if it's also in deny, it should be denied.
        let checker = make_checker(
            vec!["dangerous_skill"],
            vec!["dangerous_skill"],
            vec![],
            vec![],
        );
        assert_eq!(
            checker.check("dangerous_skill", &[]),
            PermissionResult::Denied
        );
    }

    #[test]
    fn test_deny_takes_precedence_over_safe_properties() {
        let checker = make_checker(
            vec!["blocked_skill"],
            vec![],
            vec!["safe_prop"],
            vec![],
        );
        assert_eq!(
            checker.check("blocked_skill", &["safe_prop".to_string()]),
            PermissionResult::Denied
        );
    }

    #[test]
    fn test_deny_takes_precedence_over_auto_approve() {
        let checker = make_checker(
            vec!["blocked_skill"],
            vec![],
            vec![],
            vec!["blocked_skill"],
        );
        assert_eq!(
            checker.check("blocked_skill", &[]),
            PermissionResult::Denied
        );
    }

    #[test]
    fn test_allow_overrides_deny_when_no_deny_matches() {
        // If deny doesn't match but allow does, it should be allowed.
        let checker = make_checker(
            vec!["other_skill"],
            vec!["good_skill"],
            vec![],
            vec![],
        );
        assert_eq!(
            checker.check("good_skill", &[]),
            PermissionResult::Allowed
        );
    }

    #[test]
    fn test_safe_properties_allowed() {
        let checker = make_checker(
            vec![],
            vec![],
            vec!["prop1", "prop2"],
            vec![],
        );
        assert_eq!(
            checker.check("any_skill", &["prop1".to_string(), "prop2".to_string()]),
            PermissionResult::Allowed
        );
    }

    #[test]
    fn test_safe_properties_with_unsafe_property_falls_through() {
        let checker = make_checker(
            vec![],
            vec![],
            vec!["safe_prop"],
            vec![],
        );
        // If any requested property is not safe, it should not be allowed via safe properties.
        assert_eq!(
            checker.check("any_skill", &["safe_prop".to_string(), "unsafe_prop".to_string()]),
            PermissionResult::Ask
        );
    }

    #[test]
    fn test_auto_approve_allowed() {
        let checker = make_checker(
            vec![],
            vec![],
            vec![],
            vec!["trusted_skill"],
        );
        assert_eq!(
            checker.check("trusted_skill", &[]),
            PermissionResult::Allowed
        );
    }

    #[test]
    fn test_auto_approve_with_prefix() {
        let checker = make_checker(
            vec![],
            vec![],
            vec![],
            vec!["trusted_*"],
        );
        assert_eq!(
            checker.check("trusted_skill_1", &[]),
            PermissionResult::Allowed
        );
        assert_eq!(
            checker.check("trusted_skill_2", &[]),
            PermissionResult::Allowed
        );
        assert_eq!(
            checker.check("untrusted_skill", &[]),
            PermissionResult::Ask
        );
    }

    #[test]
    fn test_no_match_falls_to_ask() {
        let checker = make_checker(
            vec!["denied_skill"],
            vec!["allowed_skill"],
            vec!["safe_prop"],
            vec!["auto_skill"],
        );
        // This skill doesn't match any rule, and requests an unsafe property.
        assert_eq!(
            checker.check("unknown_skill", &["unsafe_prop".to_string()]),
            PermissionResult::Ask
        );
    }

    #[test]
    fn test_empty_config_asks() {
        let checker = make_checker(vec![], vec![], vec![], vec![]);
        assert_eq!(
            checker.check("any_skill", &[]),
            PermissionResult::Ask
        );
    }

    #[test]
    fn test_deny_with_prefix() {
        let checker = make_checker(
            vec!["malicious_*"],
            vec![],
            vec![],
            vec![],
        );
        assert_eq!(
            checker.check("malicious_script", &[]),
            PermissionResult::Denied
        );
        assert_eq!(
            checker.check("malicious_", &[]),
            PermissionResult::Denied
        );
        assert_eq!(
            checker.check("benign_script", &[]),
            PermissionResult::Ask
        );
    }

    #[test]
    fn test_allow_with_prefix() {
        let checker = make_checker(
            vec![],
            vec!["safe_*"],
            vec![],
            vec![],
        );
        assert_eq!(
            checker.check("safe_script", &[]),
            PermissionResult::Allowed
        );
        assert_eq!(
            checker.check("unsafe_script", &[]),
            PermissionResult::Ask
        );
    }
}