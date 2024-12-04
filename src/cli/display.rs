use super::error::CliError;
use crate::audit::model::AuditResult;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};

/// Display handler for CLI output and user interaction
pub struct Display {
    /// Optional progress spinner for long-running operations
    spinner: Option<ProgressBar>,
}

impl Display {
    /// Creates a new Display instance
    pub fn new() -> Self {
        Self { spinner: None }
    }

    /// Creates and displays a spinner with the given message
    ///
    /// # Arguments
    /// * `message` - The message to display alongside the spinner
    pub fn create_spinner(&mut self, message: &str) {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner} {msg}")
                .unwrap(),
        );
        spinner.set_message(message.to_string());
        self.spinner = Some(spinner);
    }

    /// Finishes and clears the current spinner if one exists
    pub fn finish_spinner(&mut self) {
        if let Some(spinner) = self.spinner.take() {
            spinner.finish_and_clear();
        }
    }

    /// Finishes the current spinner with a final message if one exists
    ///
    /// # Arguments
    /// * `message` - The final message to display
    pub fn finish_spinner_with_message(&mut self, message: &str) {
        if let Some(spinner) = self.spinner.take() {
            spinner.finish_with_message(message.to_string());
        }
    }

    /// Displays a list of modified files with their status
    ///
    /// # Arguments
    /// * `files` - Vector of tuples containing file paths and their status
    pub fn display_files(&self, files: &Vec<(String, String)>) {
        if files.is_empty() {
            println!("> No modified files");
            return;
        }

        let max_width = 50;
        println!("╭─ Modified Files {}╮", "─".repeat(max_width - 16));
        for (file, status) in files {
            let description = match status.as_str() {
                "modified" => "\x1b[33m●\x1b[0m",
                "untracked" => "\x1b[32m○\x1b[0m",
                _ => "•",
            };
            let truncated_file = self.truncate_string(file, max_width - 4);
            println!(
                "│ {} {:<width$} │",
                description,
                truncated_file,
                width = max_width - 4
            );
        }
        println!("╰{}╯", "─".repeat(max_width));
    }

    /// Prompts the user to accept or reject a commit message
    ///
    /// # Arguments
    /// * `file` - The file being committed
    /// * `message` - The proposed commit message
    ///
    /// # Returns
    /// * `Ok(bool)` - True if accepted, false if rejected
    /// * `Err(CliError)` - If there's an IO error
    pub fn prompt_commit_message(&self, file: &str, message: &String) -> Result<bool, CliError> {
        println!("\n╭─ Commit Message Preview {}╮", "─".repeat(50 - 23));
        println!("│ File: {}", file);
        println!("│ Message: {}", message);
        println!("╰{}╯", "─".repeat(50));

        print!("Accept this commit message? [Y/n]: ");
        io::stdout().flush().map_err(CliError::IoError)?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(CliError::IoError)?;

        Ok(input.trim().to_lowercase() != "n")
    }

    /// Truncates a string to the specified maximum width
    ///
    /// # Arguments
    /// * `s` - The string to truncate
    /// * `max_width` - Maximum allowed width
    ///
    /// # Returns
    /// The truncated string with "..." appended if necessary
    fn truncate_string(&self, s: &str, max_width: usize) -> String {
        if s.len() <= max_width {
            return s.to_string();
        }
        format!("{}...", &s[..max_width - 3])
    }

    /// Displays the results of a code audit
    ///
    /// # Arguments
    /// * `results` - The audit results to display
    pub fn show_audit_results(&self, file: &str, results: &AuditResult) {
        println!("\n╭─ Code Audit Results {}╮", "─".repeat(50 - 20));

        for issue in &results.issues {
            let severity_color = match issue.severity.as_str() {
                "CRITICAL" => "\x1b[31m", // Red
                "HIGH" => "\x1b[33m",     // Yellow
                "MEDIUM" => "\x1b[32m",   // Green
                "LOW" => "\x1b[36m",      // Cyan
                _ => "",
            };

            println!(
                "│ {}[{}]\x1b[0m {}",
                severity_color,
                format!("{:?}", issue.severity),
                issue.title
            );
            println!("│ Impact: {}", issue.impact);
            println!("│ Suggestion: {}", issue.suggestion);
            println!("│ Context: {}", issue.context);
            println!("│ File: {}", file);
            println!("├{}┤", "─".repeat(48));
        }

        println!("│ Summary: {}", results.summary);
        println!("╰{}╯", "─".repeat(50));
    }
}
