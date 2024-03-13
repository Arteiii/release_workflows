use std::{fs, io};

use poem_openapi::Object;

// Define the WorkflowScripts struct
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct WorkflowScripts {
    makefile: bool,
    script: bool,
}

impl WorkflowScripts {
    fn new() -> Self {
        WorkflowScripts {
            makefile: false,
            script: false,
        }
    }

    fn set_makefile(&mut self, exists: bool) {
        self.makefile = exists;
    }

    fn set_script(&mut self, exists: bool) {
        self.script = exists;
    }

    fn has_makefile(&self) -> bool {
        self.makefile
    }

    fn has_script(&self) -> bool {
        self.script
    }
}

pub fn workflows_exist(path: &str) -> Result<WorkflowScripts, io::Error> {
    let makefile_path = format!("{}/workflows/make/Makefile", path);
    let script_path = format!("{}/workflows/script/build_script.sh", path);

    let mut scripts = WorkflowScripts::new();

    if let Ok(metadata) = fs::metadata(&makefile_path) {
        if metadata.is_file() {
            scripts.set_makefile(true);
        }
    }

    if let Ok(metadata) = fs::metadata(&script_path) {
        if metadata.is_file() {
            scripts.set_script(true);
        }
    }

    if scripts.has_makefile() || scripts.has_script() {
        Ok(scripts)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Required files not found in release workflow directory",
        ))
    }
}
