//! Blogger
//! Creates and updates blog entries for blog.ptrace.dev
//!
//! Copyright (C) 2024  worstprgr <adam@p-trace.com>  GPG Key: key.p-trace.com
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::process::exit;

use pulldown_cmark::Options;
use clap::Parser;

mod entry;
mod updater;
mod builder;
mod sites;


// TODO: Implement a "purge" option. Which deletes contents of the "www/posts" dir


// -----------
// Blog Config
// -----------
const PAGE_DIR: &str = "page";
const ENTRY_DIR: &str = "entries";
const WWW_DIR: &str = "www";
const MODULES_DIR: &str = "modules";
const POSTS_DIR: &str = "posts";
const TEMPLATE_DIR: &str = "templates";

const REPLACE_TITLE: &str = "{{ TITLE }}";
const REPLACE_DATE: &str = "{{ DATE }}";
const REPLACE_LINK: &str = "{{ ENTRY_LINK }}";
const REPLACE_CONTENT: &str = "{{ CONTENT }}";

const REPLACE_TOP: &str = "{{ TOP }}";
const REPLACE_BOT: &str = "{{ BOTTOM }}";

const REPLACE_RELATIVE_PATH: &str = "{{ RELATIVE }}";

// ---
// CLI
// ---
const ABOUT_CLI: &str = "Creates and updates blog entries";


#[derive(Parser)]
#[command(name = "Blogger")]
#[command(about = ABOUT_CLI)]
#[command(version, long_about = None)]
pub struct Cli {
    /// Creating a new blog entry
    #[arg(short, long)]
    pub new_entry: Option<String>,

    /// Update blog
    #[arg(short, long)]
    pub update_blog: bool,
}


fn main() {
    let cli = Cli::parse();

    match cli.new_entry {
        Some(v) => {
            let stripped = v.trim_end_matches(".md");
            entry::EntryManager::new()
                .new_entry(stripped.to_string());
            exit(0);
            },
        None => ()
    }

    if cli.update_blog {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_GFM);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);


        // collect all *.md entries
        let mut updater = updater::Entries::new(options);
        let r = updater.collect_entries();

        // Site: Privacy Policy
        let mut sites = sites::Sites::new();
        let privacy_policy = sites.privacy_policy();

        let mut b = builder::Builder::new(&r, &privacy_policy);
        b.build();
        exit(0);
    }

    println!("Add `-h` for commands");
}

