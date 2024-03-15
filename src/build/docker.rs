use std::process::{Command, Output};

pub struct DockerManager {
    image_name: String,
    container_name: String,
}

impl DockerManager {
    pub fn new(image_name: &str, container_name: &str) -> Result<Self, String> {
        // Check if Docker is available
        let docker_check = Command::new("docker").arg("--version").output();

        if let Ok(output) = docker_check {
            if output.status.success() {
                // Docker is available
                Ok(DockerManager {
                    image_name: image_name.to_string(),
                    container_name: container_name.to_string(),
                })
            } else {
                // Docker command failed
                Err("Docker is not available".to_string())
            }
        } else {
            // Error executing Docker command
            Err("Failed to execute Docker command".to_string())
        }
    }

    pub fn run_container(&self, options: &[&str]) -> Result<(), String> {
        // Check if the container is already running
        if self.is_container_running()? {
            return Ok(());
        }

        // Start the container if it's not running
        self.start_container(options)
    }

    pub fn stop_container(&self) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["stop", &self.container_name])
            .output()
            .map_err(|e| format!("Failed to stop Docker container: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "Failed to stop Docker container: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    pub fn build_image(&self, dockerfile_content: &str) -> Result<Output, String> {
        let temp_dir =
            tempfile::tempdir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
        let dockerfile_path = temp_dir.path().join("Dockerfile");
        std::fs::write(&dockerfile_path, dockerfile_content)
            .map_err(|e| format!("Failed to write Dockerfile: {}", e))?;

        Command::new("docker")
            .args(&[
                "build",
                "-t",
                &self.image_name,
                "-f",
                dockerfile_path.to_str().unwrap(),
                ".",
            ])
            .current_dir(temp_dir.path())
            .output()
            .map_err(|e| format!("Failed to execute build command: {}", e))
    }

    /// Method to pull the Docker image if not found locally
    fn pull_image_if_not_exists(&self) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&["images", "--format", "{{.Repository}}:{{.Tag}}"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        if !output_str.contains(&self.image_name) {
            println!("Pulling Docker image {}...", &self.image_name);
            let pull_output = Command::new("docker")
                .args(&["pull", &self.image_name])
                .output()
                .map_err(|e| format!("Failed to pull Docker image: {}", e))?;

            if !pull_output.status.success() {
                return Err(format!(
                    "Failed to pull Docker image: {}",
                    String::from_utf8_lossy(&pull_output.stderr)
                ));
            }
        }

        Ok(())
    }

    fn start_container(&self, options: &[&str]) -> Result<(), String> {
        let mut cmd = Command::new("docker");
        cmd.arg("start");

        // Specify the container name
        cmd.arg(&self.container_name);

        // Execute the command
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute start command: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            // Attempt to pull the image if the start command failed
            if let Err(err) = self.pull_image_if_not_exists() {
                return Err(format!("Failed to pull Docker image: {}", err));
            }

            // Retry starting the container
            let output_retry = Command::new("docker")
                .arg("start")
                .arg(&self.container_name)
                .output()
                .map_err(|e| {
                    format!("Failed to execute start command after pulling image: {}", e)
                })?;

            if output_retry.status.success() {
                tracing::info!("Container is now running.");
                Ok(())
            } else {
                let err_msg = format!(
                    "Failed to start Docker container: {}",
                    String::from_utf8_lossy(&output_retry.stderr)
                );
                tracing::error!(err_msg);
                Err(err_msg)
            }
        }
    }

    /// Check if the Docker container is running
    fn is_container_running(&self) -> Result<bool, String> {
        let output = Command::new("docker")
            .args(&["ps", "--format", "{{.Names}}"])
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.contains(&self.container_name))
    }

    pub fn run_script_in_container(
        &self,
        script_path: &str,
        script_name: &str,
    ) -> Result<(), String> {
        // copy script to container
        let cp_output = Command::new("docker")
            .args(&[
                "cp",
                script_path,
                &format!("{}:{}", &self.container_name, script_name),
            ])
            .output()
            .map_err(|e| format!("Failed to copy script to container: {}", e))?;

        if !cp_output.status.success() {
            return Err(format!(
                "Failed to copy script to container: {}",
                String::from_utf8_lossy(&cp_output.stderr)
            ));
        }

        // execute script
        let exec_output = Command::new("docker")
            .args(&["exec", &self.image_name, "sh", script_name])
            .output()
            .map_err(|e| format!("Failed to execute script in container: {}", e))?;

        if !exec_output.status.success() {
            return Err(format!(
                "Script execution failed: {}",
                String::from_utf8_lossy(&exec_output.stderr)
            ));
        }

        Ok(())
    }

    pub fn copy_from_container(&self, container_path: &str, host_path: &str) -> Result<(), String> {
        let output = Command::new("docker")
            .args(&[
                "cp",
                &format!("{}:{}", &self.container_name, container_path),
                host_path,
            ])
            .output()
            .map_err(|e| format!("Failed to copy file from container: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to copy file from container: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    pub fn clone_repository(
        &self,
        repository_url: &str,
        target_directory: &str,
    ) -> Result<Output, String> {
        Command::new("docker")
            .args(&[
                "exec",
                &self.container_name,
                "git",
                "clone",
                repository_url,
                target_directory,
            ])
            .output()
            .map_err(|e| format!("Failed to clone repository: {}", e))
    }

    pub fn run_command_in_container(&self, command: &str) -> Result<(), String> {
        let exec_output = Command::new("docker")
            .args(&["exec", &self.container_name, "sh", "-c", command])
            .output()
            .map_err(|e| format!("Failed to execute command in container: {}", e))?;

        if !exec_output.status.success() {
            return Err(format!(
                "Command execution failed: {}",
                String::from_utf8_lossy(&exec_output.stderr)
            ));
        }

        Ok(())
    }
}
