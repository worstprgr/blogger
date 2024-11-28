use std::path::PathBuf;
use std::fs;
use std::process::exit;

use crate::{
    PAGE_DIR,
    WWW_DIR,
    MODULES_DIR,
    POSTS_DIR,
    TEMPLATE_DIR,
    REPLACE_TITLE,
    REPLACE_DATE,
    REPLACE_LINK,
    REPLACE_CONTENT,
    REPLACE_TOP,
    REPLACE_BOT,
    REPLACE_RELATIVE_PATH,

    updater::Entry,
};


pub struct Builder {
    www_p: PathBuf,
    templates_p: PathBuf,
    entries_from_md: Vec<Entry>,
    entries_to_html: Vec<String>,
    single_entries_to_html: Vec<String>,
    overview: Vec<String>,
    privacy_policy: Vec<String>,
    entries_created: Vec<i64>,
}

impl Builder {
    pub fn new(entries: &Vec<Entry>, privacy_policy: &String) -> Builder {
        let mut builder = Builder {
            www_p: PathBuf::from(PAGE_DIR),
            templates_p: PathBuf::from(PAGE_DIR),
            entries_from_md: entries.to_vec(),
            entries_to_html: vec![],
            single_entries_to_html: vec![],
            overview: vec![],
            privacy_policy: vec![privacy_policy.to_string()],
            entries_created: vec![],
        };
        builder.www_p.push(WWW_DIR);
        builder.templates_p.push(TEMPLATE_DIR);
        builder
    }

    pub fn build(&mut self) {
        self.entries_builder();
        self.generic_builder(&self.entries_to_html, "index.html", "");
        self.generic_builder(&self.overview, "overview.html", "");
        self.generic_builder(&self.privacy_policy, "privacy-policy.html", "");
        self.page_entry_builder();
    }

    fn entries_builder(&mut self) -> &Self {
        let mut entry_file = self.templates_p.join(MODULES_DIR);
        entry_file.push("ENTRY.html");

        let mut single_entry_file = self.templates_p.join(MODULES_DIR);
        single_entry_file.push("S_ENTRY.html");

        let mut overview_file = self.templates_p.join(MODULES_DIR);
        overview_file.push("OVERVIEW.html");

        let entry_html = self.open_file(&entry_file);
        let single_entry_html = self.open_file(&single_entry_file);
        let overview_html = self.open_file(&overview_file);

        // Sort blog entries by file creation timestamp.
        //
        // This could blow up, if the file metadata is scuffed,
        // so the order could be unexpected.
        //
        // Also the performance could degrade, if there are many, many, many entries.
        // But this is a problem for a different day.
        let _ = &self.entries_from_md.sort_by(|a, b| b.created_ts.cmp(&a.created_ts));

        for entry in &self.entries_from_md {
            self.entries_created.push(entry.created_ts);

            let mut posts_url = PathBuf::new();
            posts_url.push(format!("/?e={}", &entry.created_ts.to_string()));

            let buf_entry = &entry_html
                .replace(REPLACE_TITLE, &entry.title)
                .replace(REPLACE_DATE, &entry.date)
                .replace(REPLACE_LINK, posts_url.to_str().unwrap())
                .replace(REPLACE_CONTENT, &entry.content);

            let buf_s_entry = &single_entry_html
                .replace(REPLACE_TITLE, &entry.title)
                .replace(REPLACE_DATE, &entry.date)
                .replace(REPLACE_CONTENT, &entry.content);

            let buf_overview = &overview_html
                .replace(REPLACE_TITLE, &entry.title)
                .replace(REPLACE_DATE, &entry.date)
                .replace(REPLACE_LINK, posts_url.to_str().unwrap());

            self.entries_to_html.push(buf_entry.to_string());
            self.single_entries_to_html.push(buf_s_entry.to_string());
            self.overview.push(buf_overview.to_string());
        }
        self
    }

    fn generic_builder(&self, content: &Vec<String>, output_fn: &str, relative_path: &str) -> &Self {
        let mut bot_file = self.templates_p.join(MODULES_DIR);
        bot_file.push("BOTTOM.html");

        let index_file = self.templates_p.join("index.html");

        let top_html = self.top_builder(relative_path);
        let bot_html = self.open_file(&bot_file);
        let index_html = self.open_file(&index_file);

        let mut buf = String::new();

        for item in content {
            buf.push_str(item);
        }

        let new_item = index_html
            .replace(REPLACE_TOP, &top_html)
            .replace(REPLACE_CONTENT, &buf)
            .replace(REPLACE_BOT, &bot_html);

        let mut www = PathBuf::from(PAGE_DIR);
        www.push(WWW_DIR);
        www.push(output_fn);

        self.write_file(&www, new_item);
        self
    }

    fn page_entry_builder(&self) -> &Self {
        let mut bot_file = self.templates_p.join(MODULES_DIR);
        bot_file.push("BOTTOM.html");

        let index_file = self.templates_p.join("index.html");

        let top_html = self.top_builder("../");
        let bot_html = self.open_file(&bot_file);
        let index_html = self.open_file(&index_file);

        for (index, entry) in self.single_entries_to_html.iter().enumerate() {
            let new_entry = index_html
                .replace(REPLACE_TOP, &top_html)
                .replace(REPLACE_CONTENT, &entry)
                .replace(REPLACE_BOT, &bot_html);

            let mut www_posts = PathBuf::from(PAGE_DIR);
            www_posts.push(WWW_DIR);
            www_posts.push(POSTS_DIR);
            www_posts.push(format!("{}.html", &self.entries_created[index]));

            self.write_file(&www_posts, new_entry);
        }
        self
    }

    fn top_builder(&self, relative_path: &str) -> String {
        let mut top_file = self.templates_p.join(MODULES_DIR);
        top_file.push("TOP.html");

        let top_html = self.open_file(&top_file);
        let new_top = top_html.replace(REPLACE_RELATIVE_PATH, &relative_path);

        new_top
    }

    fn write_file(&self, file_path: &PathBuf, contents: String) -> &Self {
        match fs::write(file_path, contents) {
            Ok(_) => self,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }
    }

    fn open_file(&self, file_path: &PathBuf) -> String {
        let content = match fs::read_to_string(file_path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        };
        content
    }
}
