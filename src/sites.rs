use std::path::{Path, PathBuf};
use std::process::exit;
use std::fs;

use crate::{MODULES_DIR, PAGE_DIR, TEMPLATE_DIR};


pub struct Sites {
    module_path: PathBuf,
}

impl Sites {
    pub fn new() -> Sites {
        let mut sites = Sites {
            module_path: PathBuf::from(PAGE_DIR),
        };
        sites.module_path.push(TEMPLATE_DIR);
        sites.module_path.push(MODULES_DIR);
        sites
    }

    pub fn privacy_policy(&mut self) -> String {
        let privacy_policy_path = self.module_path.join("PRIVACY_POLICY.html");
        self.read_file(&privacy_policy_path)
    }

    fn read_file(&self, fp: &Path) -> String {
        let content = match fs::read_to_string(fp) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };
        content
    }
}
