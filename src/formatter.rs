use colored::Colorize;
use colored::*;
use std::fmt::Write;
use crate::printer;


fn get_lang_color(lang: &str) -> fn(&str) -> ColoredString {
    match lang.to_lowercase().as_str() {
        "rust" | "rs" => |s: &str| s.white(),
        "zsh" => |s| s.white(),
        "md" | "markdown" => |s| s.white(),
        "bash" => |s| s.white(),
        "python" | "py" => |s| s.blue(),
        "c" => |s| s.blue(),
        "go" | "golang" => |s| s.blue(),
        "php" => |s| s.purple(),
        "asm" | "assembly" | "assemblygas" => |s| s.magenta(),
        "c#" | "csharp" => |s| s.magenta(),
        "java" => |s| s.red(),
        "ruby" => |s| s.red(),
        "cpp" | "c++" | "cheader" | "cppheader" => |s| s.blue(),
        "javascript" | "js" => |s| s.bright_yellow(),
        "typescript" | "ts" => |s| s.bright_blue(),
        "kotlin" | "kt" => |s| s.truecolor(255, 165, 0),

        _ => |s| s.white(),
    }
}

pub fn printer(stats: &crate::info::RepoStats, top_lang: &str) {
    let color_func = get_lang_color(top_lang);

    let key = |name: &str| color_func(name).bold();

    let mut buffer = String::with_capacity(512);

    let _ = writeln!(buffer, "{}: {} ({})", key("Head"), stats.id, stats.branch);
    let _ = writeln!(buffer, "{}: {}", key("Repository name"), stats.repo_name);
    let _ = writeln!(buffer, "{}: {}", key("Authors"), stats.authors_str);
    let _ = writeln!(buffer, "{}: {}", key("Files"), stats.total_files);
    let _ = writeln!(buffer, "{}: {}", key("Markdown files"), stats.md_files_count);
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
}