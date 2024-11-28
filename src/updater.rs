use std::path::{PathBuf, Path};
use std::fs;
use std::process::exit;

use filetime::FileTime;
use walkdir::{DirEntry, WalkDir};
use pulldown_cmark::{html::push_html, Options, Parser};
use crate::{ENTRY_DIR, PAGE_DIR};


#[derive(Debug, Clone)]
pub struct Entry {
    pub title: String,
    pub date: String,
    pub content: String,
    pub created_ts: i64,
}

impl Entry {
    fn new(title: String, date: String, content: String, created_ts: i64) -> Entry {
        let entry = Entry {
            title,
            date,
            content,
            created_ts,
        };
        entry
    }
}

#[derive(Debug)]
pub struct Entries {
    pub entries_path: PathBuf,
    pub entries: Vec<Entry>,
    pub options: Options,
}

impl Entries {
    pub fn new(options: Options) -> Entries {
        let mut updater = Entries {
            entries_path: PathBuf::from(PAGE_DIR),
            entries: vec![],
            options,
        };
        updater.entries_path.push(ENTRY_DIR);
        updater
    }

    pub fn collect_entries(&mut self) -> &Vec<Entry> {
        let entry_dir = WalkDir::new(&self.entries_path);

        for entry in entry_dir.into_iter().filter_map(Result::ok)
            .filter(|e| {
                let path = e.path();
                path.is_file() && path.extension().map(|ext| ext == "md").unwrap_or(false)
            })
        {
            let html = self.md_to_html(entry.path(), self.options);

            let created = match entry.metadata() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            };

            let ctime = FileTime::from_last_modification_time(&created).seconds();

            let some_entry = Entry::new(entry.file_name()
                .to_string_lossy()
                .to_string()
                .replace(".md", ""),
                self.get_date(entry),
                html,
                ctime);
            self.entries.push(some_entry);
        }
        &self.entries
    }

    fn md_to_html(&self, md_fp: &Path, options: Options) -> String {
        let mut html_content = String::new();
        let md_content = self.read_md_file(md_fp);
        let parser_content = Parser::new_ext(&md_content, options);
        push_html(&mut html_content, parser_content);
        html_content
    }

    fn read_md_file(&self, md_fp: &Path) -> String {
        let content = match fs::read_to_string(md_fp) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };
        content
    }

    fn get_date(&self, entry: DirEntry) -> String {
        let p = entry.path();

        let day_tmp = p.parent().unwrap();
        let month_tmp = day_tmp.parent().unwrap();
        let year_tmp = month_tmp.parent().unwrap();

        let day = &day_tmp.file_name().unwrap().to_str().unwrap();
        let month = &month_tmp.file_name().unwrap().to_str().unwrap();
        let year = &year_tmp.file_name().unwrap().to_str().unwrap();

        format!("{}/{}/{}", day, month, year)
    }
}
