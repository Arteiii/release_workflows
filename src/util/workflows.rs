use std::fs;

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
}

pub fn workflows_exist(path: &str) -> Option<WorkflowScripts> {
    let release_workflow_path = format!("{}/workflows", path);

    if let Ok(entries) = fs::read_dir(release_workflow_path) {
        let mut scripts = WorkflowScripts::new();

        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.eq_ignore_ascii_case("make") {
                        scripts.set_makefile(true);
                    } else if file_name.eq_ignore_ascii_case("script") {
                        scripts.set_script(true);
                    }
                }
            }
        }

        Some(scripts)
    } else {
        None
    }
}
