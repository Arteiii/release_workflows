use std::process::{Command, Output};

pub struct DockerManager {
    image_name: String,
}

impl DockerManager {
    pub fn new(image_name: &str) -> Result<Self, String> {
        // check docker
        let docker_check = Command::new("docker").arg("--version").output();

        if let Ok(output) = docker_check {
            if output.status.success() {
                // docker is available
                Ok(DockerManager {
                    image_name: image_name.to_string(),
                })
            } else {
                // docker command failed
                Err("Docker is not available".to_string())
            }
        } else {
            // error executing docker command
            Err("Failed to execute Docker command".to_string())
        }
    }

    pub fn build_image(&self, dockerfile_path: &str, context_path: &str) -> Result<Output, String> {
        Command::new("docker")
            .args(&[
                "build",
                "-t",
                &self.image_name,
                "-f",
                dockerfile_path,
                context_path,
            ])
            .output()
            .map_err(|e| format!("Failed to execute build command: {}", e))
    }

    pub fn run_container(&self, options: &[&str]) -> Result<Output, String> {
        Command::new("docker")
            .args(&["run"])
            .args(options)
            .args(&[&self.image_name])
            .output()
            .map_err(|e| format!("Failed to execute run command: {}", e))
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
                &format!("{}:{}", &self.image_name, script_name),
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
                &format!("{}:{}", &self.image_name, container_path),
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
}
