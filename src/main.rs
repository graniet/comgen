mod audit;
mod cli;
mod config;
mod git;
mod providers;
mod utils;

use audit::parser::parse_audit_response;
use cli::{Cli, Display};
use config::Config;
use git::GitCommands;
use providers::AIProvider;
use std::io::{self, Write};
use tracing::info;
use utils::{Logger, Spinner};

/// Process a single modified file by generating a commit message and optionally performing an audit
///
/// # Arguments
/// * `file` - The path to the file being processed
/// * `provider` - The AI provider used for generating responses
/// * `config` - The application configuration
/// * `cli` - The parsed command line arguments
/// * `spinner` - Progress spinner for visual feedback
/// * `display` - Display utility for user interaction
///
/// # Returns
/// * `Ok(())` - If the file was processed successfully
/// * `Err(Box<dyn std::error::Error>)` - If an error occurred during processing
fn process_file(
    file: &str,
    provider: &Box<dyn AIProvider>,
    config: &Config,
    cli: &Cli,
    spinner: &mut Spinner,
    display: &Display,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if file is deleted
    let is_deleted = GitCommands::get_modified_files()?
        .iter()
        .find(|(f, status)| f == file && status == "deleted")
        .is_some();

    if is_deleted {
        spinner.finish_with_message(&format!("✗ File {} has been deleted", file));
        return Ok(());
    }

    // Get diff for specific file
    let file_diff = match GitCommands::get_diff(Some(file)) {
        Ok(diff) => diff,
        Err(e) => {
            spinner.finish_with_message(&format!("✗ Error getting diff for {}", file));
            return Err(e.into());
        }
    };

    // Generate specific prompt for this file
    let commit_types = config.templates.commit_types.join("\n- ");
    let output_format = &config.templates.output_format;

    let file_prompt = config.base_prompt.replace("{{git_diff}}", &file_diff)
        + "\n\nAllowed commit types:\n- "
        + &commit_types
        + "\n\nOutput format requirements:"
        + "\n- Follow this template: "
        + &output_format.template
        + "\n- Maximum length: "
        + &output_format.max_length.to_string()
        + " characters"
        + "\n\nExample commits:\n- "
        + &output_format.examples.join("\n- ");

    // Perform audit first if enabled
    let mut skip_audit = false;
    if config.audit.enabled {
        spinner.start("Performing code audit...");

        let audit_prompt = config.audit.prompt.replace("{{git_diff}}", &file_diff);
        match provider.generate_response(&audit_prompt) {
            Ok(audit_response) => {
                let audit_result = parse_audit_response(&audit_response)?;
                display.show_audit_results(file, &audit_result);
            }
            Err(e) => {
                spinner.finish_with_message("✗ Error during audit");
                print!("Audit failed. Continue anyway? [y/N]: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "y" {
                    skip_audit = true;
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    if !skip_audit {
        spinner.start("Generating commit message...");
    }

    // Generate commit message for this file
    match provider.generate_response(&file_prompt) {
        Ok(mut file_response) => {
            if !cli.prefix.is_empty() {
                file_response = format!("[{}] {}", cli.prefix, file_response);
            }

            spinner.finish();

            let mut accepted = false;
            let mut current_message = file_response;

            while !accepted {
                match display.prompt_commit_message(file, &current_message) {
                    Ok(true) => {
                        accepted = true;
                    }
                    Ok(false) => {
                        spinner.start("Generating new commit message...");

                        match provider.generate_response(&file_prompt) {
                            Ok(new_response) => {
                                current_message = if !cli.prefix.is_empty() {
                                    format!("[{}] {}", cli.prefix, new_response)
                                } else {
                                    new_response
                                };
                                spinner.finish();
                            }
                            Err(e) => {
                                spinner.finish_with_message("✗ Error generating new message");
                                return Err(e.into());
                            }
                        }
                    }
                    Err(e) => return Err(e.into()),
                }
            }

            spinner.start(&format!("Committing {}", file));

            // Stage and commit with accepted message
            GitCommands::stage_file(file)?;
            GitCommands::commit(&current_message)?;

            if cli.auto_push {
                match GitCommands::push() {
                    Ok(_) => spinner.finish_with_message(&format!("✓ Pushed {}", file)),
                    Err(e) => {
                        spinner.finish_with_message(&format!("✗ Error pushing {}", file));
                        return Err(e.into());
                    }
                }
            } else {
                spinner.finish_with_message(&format!("✓ Committed {}", file));
            }
            Ok(())
        }
        Err(e) => {
            spinner.finish_with_message(&format!("✗ Error generating message for {}", file));
            Err(e.into())
        }
    }
}

/// Main entry point for the application
///
/// Sets up logging, loads configuration, and processes modified files in the git repository
///
/// # Returns
/// * `Ok(())` - If all operations completed successfully
/// * `Err(Box<dyn std::error::Error>)` - If an error occurred during execution
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    Logger::setup()?;

    // Parse CLI arguments
    let cli = Cli::parse_args();
    let config_path = cli.get_config_path()?;
    info!("loading config from {}", config_path);

    // Load configuration
    let mut config = config::load_config(&config_path)?;

    // Try to load local template if it exists
    if let Err(e) = config.load_local_template() {
        info!("No local template found or error loading it: {}", e);
    } else {
        info!("Local template loaded successfully");
    }

    info!("using provider: {}", config.provider);
    info!("using model: {}", config.model);

    // Create provider
    let provider = providers::create_provider(&config.provider, &config);

    // Initialize display utilities
    let mut spinner = Spinner::new();
    let display = Display::new();

    // Get modified files
    spinner.start("Analyzing repository...");
    let modified_files = GitCommands::get_modified_files()?;
    spinner.finish();

    display.display_files(&modified_files);

    let files_list: Vec<String> = modified_files
        .iter()
        .map(|(file, _)| file.clone())
        .collect();

    spinner.start("Processing files...");

    // Process each file
    for file in &files_list {
        if let Err(e) = process_file(file, &provider, &config, &cli, &mut spinner, &display) {
            return Err(e);
        }
    }

    spinner.finish();
    Ok(())
}
