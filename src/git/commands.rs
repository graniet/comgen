use super::error::GitError;
use std::process::Command;

/// Provides Git command functionality through system commands
pub struct GitCommands;

impl GitCommands {
    /// Gets a list of modified files in the Git repository
    ///
    /// # Returns
    /// * `Ok(Vec<(String, String)>)` - List of (file path, status) tuples where status is "deleted" or "modified"
    /// * `Err(GitError)` - If Git command execution fails
    pub fn get_modified_files() -> Result<Vec<(String, String)>, GitError> {
        let deleted = Command::new("git")
            .args(["ls-files", "--deleted"])
            .output()?;

        let unstaged = Command::new("git")
            .args(["ls-files", "--modified"])
            .output()?;

        let mut files = Vec::new();

        if deleted.status.success() {
            files.extend(
                String::from_utf8(deleted.stdout)?
                    .lines()
                    .map(|s| (s.to_string(), "deleted".to_string())),
            );
        }

        if unstaged.status.success() {
            let deleted_files: Vec<String> = files.iter().map(|(path, _)| path.clone()).collect();
            files.extend(
                String::from_utf8(unstaged.stdout)?
                    .lines()
                    .filter(|s| !deleted_files.contains(&s.to_string()))
                    .map(|s| (s.to_string(), "modified".to_string())),
            );
        }

        // if untracked.status.success() {
        //     files.extend(
        //         String::from_utf8(untracked.stdout)?
        //             .lines()
        //             .map(|s| (s.to_string(), "untracked".to_string()))
        //     );
        // }

        Ok(files)
    }

    /// Gets the diff output for a specific file or all files
    ///
    /// # Arguments
    /// * `file` - Optional file path to get diff for. If None, gets diff for all files
    ///
    /// # Returns
    /// * `Ok(String)` - The diff output
    /// * `Err(GitError)` - If Git command execution fails
    pub fn get_diff(file: Option<&str>) -> Result<String, GitError> {
        let mut command = Command::new("git");
        command.arg("diff");

        if let Some(file_path) = file {
            command.arg(file_path);
        }

        let output = command.output()?;

        if !output.status.success() {
            return Err(GitError::DiffError(
                "Failed to execute git diff".to_string(),
            ));
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    /// Stages a file for commit
    ///
    /// # Arguments
    /// * `file` - Path of the file to stage
    ///
    /// # Returns
    /// * `Ok(())` - If staging succeeds
    /// * `Err(GitError)` - If Git command execution fails
    pub fn stage_file(file: &str) -> Result<(), GitError> {
        let output = Command::new("git").args(["add", file]).output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::StageError(format!(
                "Failed to stage file {}: {}",
                file, error_msg
            )));
        }

        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Ok(())
    }

    /// Commits staged changes with a message
    ///
    /// # Arguments
    /// * `message` - The commit message
    ///
    /// # Returns
    /// * `Ok(())` - If commit succeeds
    /// * `Err(GitError)` - If Git command execution fails
    pub fn commit(message: &str) -> Result<(), GitError> {
        let output = Command::new("git")
            .args(["commit", "-q", "-m", message])
            .output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::CommitError(format!(
                "Git commit failed: {}",
                error_msg
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.contains("file changed")
            && !stdout.contains("insertion")
            && !stdout.contains("deletion")
        {
            println!("{}", stdout);
        }
        Ok(())
    }

    /// Pushes commits to the remote repository
    ///
    /// # Returns
    /// * `Ok(())` - If push succeeds
    /// * `Err(GitError)` - If Git command execution fails
    pub fn push() -> Result<(), GitError> {
        let output = Command::new("git").args(["push", "-q"]).output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::PushError(format!(
                "Git push failed: {}",
                error_msg
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            println!("{}", stdout);
        }
        Ok(())
    }
}
