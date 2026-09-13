mod info;
mod logos;
mod printer;
mod formatter;

use git2::Repository;
use crate::info::get_info;

fn main() -> Result<(), git2::Error> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(_) => {
            eprintln!("Error!!! In your Project not found git!.");
            std::process::exit(1);
        }
    };
    let stats = get_info(&repo)?;

    let top_lang = if let Some((lang, _)) = stats.sorted_langs.first() {
        lang.as_str()
    } else if stats.md_files_count > 0 {
        "Markdown"
    } else {
        "unknown"
    };

    formatter::printer(&stats, top_lang);
    Ok(())
}