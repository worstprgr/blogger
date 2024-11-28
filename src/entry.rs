use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::process::{Command, exit};
use chrono::{DateTime, Utc};

use crate::{PAGE_DIR, ENTRY_DIR};


pub struct EntryManager {
    entries_path: PathBuf,
    current_date: DateTime<Utc>,
    current_dir: PathBuf,
}

impl EntryManager {
    pub fn new() -> EntryManager {
        let mut entry_manager = EntryManager {
            entries_path: PathBuf::from(PAGE_DIR),
            current_date: Utc::now(),
            current_dir: PathBuf::new(),
        };
        entry_manager.entries_path.push(ENTRY_DIR);
        entry_manager
    }

    pub fn new_entry(&mut self, md_fn: String) -> &Self {
        self.create_dirs();
        self.current_dir.push(format!("{}.md", &md_fn));

        match &self.current_dir.try_exists() {
            Ok(v) => if *v {
                eprintln!("File exists in target directory");
                exit(1);
            },
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }

        let mut file = match fs::File::create(&self.current_dir) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };

        match file.write_all(b"") {
            Ok(_) => println!("Created file"),
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };

        self.open_editor();
        self
    }

    fn open_editor(&self) -> &Self {
        // Check if nvim exists
        let output = Command::new("nvim")
            .args(["--version"])
            .output()
            .expect("Cannot find neovim");

        let neovim_magic_bytes = vec![78, 86, 73, 77];

        if output.stdout[0..4] != neovim_magic_bytes {
            eprintln!("Cannot find neovim");
            exit(1);
        }

        #[cfg(target_family = "windows")]
        let _ = Command::new("wt")
            .args(["-p", "urmom", "nvim", &self.current_dir.to_str().unwrap()])
            .spawn()
            .expect("Cannot start neovim or/and open new terminal tab");

        #[cfg(target_family = "unix")]
        todo!("Spawn a new terminal tab with nvim");
        // let _ = Command::new("")
        //     .args(["foo", "nvim", &self.current_dir.to_str().unwrap()])
        //     .spawn()
        //     .expect("Cannot start neovim or/and open new terminal tab");

        self
    }

    fn create_dirs(&mut self) -> &Self {
        let year = self.current_date.format("%Y").to_string();
        let month = self.current_date.format("%m").to_string();
        let day = self.current_date.format("%d").to_string();
        self.entries_path.push(year);
        self.entries_path.push(month);
        self.entries_path.push(day);

        self.current_dir.push(&self.entries_path);

        match fs::create_dir_all(&self.entries_path) {
            Ok(_) => println!("Created dirs"),
            Err(_) => println!("Dirs are existing, ignoring ...")
        }
        self
    }
}
