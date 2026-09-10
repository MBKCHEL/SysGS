mod info;
mod logos;
mod printer;

use std::fmt::Write;
use git2::Repository;

fn main() -> Result<(), git2::Error> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(_) => {
            eprintln!("Error!!! In your Project not found git!.");
            std::process::exit(1);
        }
    };

    let stats = info::get_info(&repo)?;

    let mut buffer = String::with_capacity(512);

    let _ = writeln!(buffer, "Head: {} ({})", stats.id, stats.branch);
    let _ = writeln!(buffer, "Authors: {}", stats.authors_str);
    let _ = writeln!(buffer, "Files: {}", stats.total_files);
    let _ = writeln!(buffer, "Lines of code: {}", stats.total_code_lines);

    let _ = write!(buffer, "Languages: ");
    let mut first = true;
    for (lang, lang_code_lines) in &stats.sorted_langs {
        let percentage: f64 = if stats.total_code_lines > 0 {
            (*lang_code_lines as f64 / stats.total_code_lines as f64) * 100.0
        } else {
            0.0
        };

        if !first {
            let _ = write!(buffer, ", ");
        }
        let _ = write!(buffer, "{} ({:.1}%)", lang, percentage);
        first = false;
    }
    let _ = writeln!(buffer);

    let _ = writeln!(buffer, "Total commits: {}", stats.total_commits);
    let _ = write!(buffer, "Last commit: {}", stats.summary);


    let top_lang = stats
        .sorted_langs
        .first()
        .map(|(lang, _)| lang.as_str())
        .unwrap_or("unknown");


    printer::render(top_lang, &buffer);

    Ok(())
}