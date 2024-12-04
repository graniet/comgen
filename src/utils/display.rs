use indicatif::{ProgressBar, ProgressStyle};

/// A spinner that can be used to show progress for long-running operations
pub struct Spinner {
    /// The underlying progress bar that drives the spinner animation
    progress_bar: Option<ProgressBar>,
}

impl Spinner {
    /// Creates a new spinner instance
    pub fn new() -> Self {
        Self { progress_bar: None }
    }

    /// Starts the spinner with the given message
    ///
    /// # Arguments
    /// * `message` - The message to display next to the spinner
    pub fn start(&mut self, message: &str) {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner} {msg}")
                .unwrap(),
        );
        spinner.set_message(message.to_string());
        self.progress_bar = Some(spinner);
    }

    /// Stops and clears the spinner
    pub fn finish(&mut self) {
        if let Some(spinner) = self.progress_bar.take() {
            spinner.finish_and_clear();
        }
    }

    /// Stops the spinner and displays a final message
    ///
    /// # Arguments
    /// * `message` - The message to display when finishing
    pub fn finish_with_message(&mut self, message: &str) {
        if let Some(spinner) = self.progress_bar.take() {
            spinner.finish_with_message(message.to_string());
        }
    }

    /// Updates the message shown next to the spinner
    ///
    /// # Arguments
    /// * `message` - The new message to display
    pub fn update_message(&self, message: &str) {
        if let Some(spinner) = &self.progress_bar {
            spinner.set_message(message.to_string());
        }
    }
}

/// Utility functions for string manipulation
pub struct StringUtils;

impl StringUtils {
    /// Truncates a string to the specified maximum width, adding "..." if truncated
    ///
    /// # Arguments
    /// * `s` - The string to truncate
    /// * `max_width` - The maximum allowed width of the string
    ///
    /// # Returns
    /// The truncated string with "..." appended if it was shortened
    pub fn truncate(s: &str, max_width: usize) -> String {
        if s.len() <= max_width {
            return s.to_string();
        }
        format!("{}...", &s[..max_width - 3])
    }
}
