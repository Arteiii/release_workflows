use std::{fs, io, path::Path};

use poem_openapi::Object;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct WorkflowScripts {
    makefile: bool,
    script: bool,
    cargo_toml: bool,
}

impl WorkflowScripts {
    fn new() -> Self {
        WorkflowScripts {
            makefile: false,
            script: false,
            cargo_toml: false,
        }
    }

    fn set_makefile(&mut self, exists: bool) {
        self.makefile = exists;
    }

    fn set_script(&mut self, exists: bool) {
        self.script = exists;
    }

    fn set_cargo_toml(&mut self, exists: bool) {
        self.cargo_toml = exists;
    }

    pub fn has_makefile(&self) -> bool {
        self.makefile
    }

    pub fn has_script(&self) -> bool {
        self.script
    }

    pub fn has_cargo_toml(&self) -> bool {
        self.cargo_toml
    }

    pub fn get_makefile_path(path: &str) -> String {
        format!("{}/workflows/make/Makefile", path)
    }

    pub fn get_script_path(path: &str) -> String {
        format!("{}/workflows/script/build_script.sh", path)
    }

    pub fn get_cargo_toml_path(path: &str) -> String {
        format!("{}/Cargo.toml", path)
    }
}

pub fn workflows_exist(path: &str) -> Result<WorkflowScripts, io::Error> {
    let mut scripts = WorkflowScripts::new();

    // Check if Makefile exists
    if let Ok(metadata) = fs::metadata(&WorkflowScripts::get_makefile_path(path)) {
        if metadata.is_file() {
            scripts.set_makefile(true);
        }
    }

    // Check if build_script.sh exists
    if let Ok(metadata) = fs::metadata(&WorkflowScripts::get_script_path(path)) {
        if metadata.is_file() {
            scripts.set_script(true);
        }
    }

    // Check if Cargo.toml exists
    if Path::new(&WorkflowScripts::get_cargo_toml_path(path)).exists() {
        scripts.set_cargo_toml(true);
    }

    if scripts.has_makefile() || scripts.has_script() || scripts.has_cargo_toml() {
        Ok(scripts)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Required files not found in release workflow directory",
        ))
    }
}
