mod info;
mod logos;
mod printer;

use colored::*;
use git2::Repository;
use std::fmt::Write;

fn get_lang_color(lang: &str) -> Color {
    match lang.to_lowercase().as_str() {
        "rust" | "rs" => Color::White,
        "python" | "py" => Color::BrightCyan,
        "c" => Color::Blue,
        "asm" | "assembly" | "assemblygas" => Color::Magenta,
        "c#" | "csharp" => Color::Magenta,
        "java" => Color::Red,
        "cpp" | "c++" | "cheader" | "cppheader" => Color::Cyan,
        "javascript" | "js" => Color::BrightYellow,
        "typescript" | "ts" => Color::BrightBlue,
        _ => Color::White,
    }
}

fn main() -> Result<(), git2::Error> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(_) => {
            eprintln!("Error!!! In your Project not found git!.");
            std::process::exit(1);
        }
    };

    let stats = info::get_info(&repo)?;

    let top_lang = stats
        .sorted_langs
        .first()
        .map(|(lang, _)| lang.as_str())
        .unwrap_or("unknown");

    let theme_color = get_lang_color(top_lang);

    let key = |name: &str| name.color(theme_color).bold();

    let mut buffer = String::with_capacity(512);

    let _ = writeln!(buffer, "{}: {} ({})", key("Head"), stats.id, stats.branch);
    let _ = writeln!(buffer, "{}: {}", key("Repository name"), stats.repo_name);
    let _ = writeln!(buffer, "{}: {}", key("Authors"), stats.authors_str);
    let _ = writeln!(buffer, "{}: {}", key("Files"), stats.total_files);
    let _ = writeln!(buffer, "{}: {}", key("Lines of code"), stats.total_code_lines);

    let _ = write!(buffer, "{}: ", key("Languages"));
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

    let _ = writeln!(buffer, "{}: {}", key("Total commits"), stats.total_commits);
    let _ = writeln!(buffer, "{}: {}", key("Last commit"), stats.summary);
    let _ = writeln!(buffer, "{}: {}", key("Repo age"), stats.age);

    printer::render(top_lang, &buffer);

    Ok(())
}