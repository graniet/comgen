use serde::{Deserialize, Serialize};

/// Contains the results of a code audit
#[derive(Debug)]
pub struct AuditResult {
    /// List of issues found during the audit
    pub issues: Vec<AuditIssueJson>,
    /// Overall summary of the audit findings
    pub summary: String,
}

impl AuditResult {
    /// Checks if there are any critical severity issues
    ///
    /// # Returns
    /// `true` if there are any critical issues, `false` otherwise
    pub fn has_critical_issues(&self) -> bool {
        self.issues.iter().any(|issue| issue.severity == "CRITICAL")
    }
}

/// Represents a single issue found during code audit in JSON format
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditIssueJson {
    pub severity: String,
    pub title: String,
    pub impact: String,
    pub context: String,
    pub suggestion: String,
}
