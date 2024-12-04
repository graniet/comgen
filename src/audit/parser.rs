use super::model::{AuditIssueJson, AuditResult};

/// Parses an audit response string into an AuditResult
///
/// # Arguments
///
/// * `response` - The raw audit response string to parse
/// * `file` - The name of the file being audited
///
/// # Returns
///
/// Returns a Result containing either:
/// * An AuditResult with the parsed issues and summary
/// * An error if parsing fails
///
/// # Example
///
/// ```
/// let response = "CRITICAL: Security vulnerability found\nMEDIUM: Code smell detected";
/// let result = parse_audit_response(response, "src/main.rs")?;
/// ```
pub fn parse_audit_response(response: &str) -> Result<AuditResult, Box<dyn std::error::Error>> {
    let mut json_response: String;

    json_response = response.replace("```json", "");
    json_response = json_response.replace("```", "");
    let issues: Vec<AuditIssueJson> = serde_json::from_str(&json_response)?;

    let issue_count = issues.len();
    Ok(AuditResult {
        issues,
        summary: format!("Found {} issues", issue_count),
    })
}
