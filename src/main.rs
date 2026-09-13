mod info;
mod logos;
mod printer;

use colored::*;
use git2::Repository;
use std::fmt::Write;

fn get_lang_color(lang: &str) -> Box<dyn Fn(&str) -> ColoredString> {
    match lang.to_lowercase().as_str() {
        "rust" | "rs" => Box::new(|s| s.white()),
        "zsh" => Box::new(|s| s.white()),
        "md" | "markdown" => Box::new(|s| s.white()),
        "bash" => Box::new(|s| s.white()),
        "python" | "py" => Box::new(|s| s.blue()),
        "c" => Box::new(|s| s.blue()),
        "go" | "golang" => Box::new(|s| s.blue()),
        "php" => Box::new(|s| s.purple()),
        "asm" | "assembly" | "assemblygas" => Box::new(|s| s.magenta()),
        "c#" | "csharp" => Box::new(|s| s.magenta()),
        "java" => Box::new(|s| s.red()),
        "ruby" => Box::new(|s| s.red()),
        "cpp" | "c++" | "cheader" | "cppheader" => Box::new(|s| s.blue()),
        "javascript" | "js" => Box::new(|s| s.bright_yellow()),
        "typescript" | "ts" => Box::new(|s| s.bright_blue()),
        "kotlin" | "kt" => Box::new(|s| s.truecolor(255, 165, 0)),

        _ => Box::new(|s| s.white()),
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

    let top_lang = if let Some((lang, _)) = stats.sorted_langs.first() {
        lang.as_str()
    } else if stats.md_files_count > 0 {
        "Markdown"
    } else {
        "unknown"
    };
    let color_func = get_lang_color(top_lang);

    let key = |name: &str| color_func(name).bold();

    let mut buffer = String::with_capacity(512);

    let _ = writeln!(buffer, "{}: {} ({})", key("Head"), stats.id, stats.branch);
    let _ = writeln!(buffer, "{}: {}", key("Repository name"), stats.repo_name);
    let _ = writeln!(buffer, "{}: {}", key("Authors"), stats.authors_str);
    let _ = writeln!(buffer, "{}: {}", key("Files"), stats.total_files);
    let _ = writeln!(buffer, "{}: {}", key("MD files"), stats.md_files_count);
    let _ = writeln!(buffer, "{}: {}", key("Txt files"), stats.text_files_count);
    let _ = writeln!(buffer, "{}: {}", key("Lines of code"), stats.total_code_lines);

    if (stats.sorted_langs.len() == 1 && stats.sorted_langs[0].0 == "Markdown")
        || (stats.sorted_langs.is_empty() && stats.md_files_count > 0) {
        let _ = writeln!(buffer, "{}: Markdown (100.0%)", key("Languages"));
    } else if stats.sorted_langs.is_empty() {
        let _ = writeln!(buffer, "{}: None", key("Languages"));
    } else {
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
    }

    let _ = writeln!(buffer, "{}: {}", key("Total commits"), stats.total_commits);
    let _ = writeln!(buffer, "{}: {}", key("Last commit"), stats.summary);
    let _ = writeln!(buffer, "{}: {}", key("Repo age"), stats.age_old);
    let _ = writeln!(buffer, "{}: {}", key("Last changes"), stats.last_change);

    printer::render(top_lang, &buffer);

    Ok(())
}