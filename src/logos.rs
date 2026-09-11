use colored::*;

pub fn get_logo(lang: &str) -> (Vec<ColoredString>, usize, fn(&str) -> ColoredString) {
    let lang_lower = lang.to_lowercase();

    let (raw_logo, color_func): (&str, fn(&str) -> ColoredString) = match lang_lower.as_str() {
        "rust" => (include_str!("../assets/rust.txt"), |s| s.white().bold()),
        "c" => (include_str!("../assets/c.txt"), |s| s.blue().bold()),
        "c++" | "cpp" => (include_str!("../assets/cpp.txt"), |s| s.blue().bold()),
        "typescript" | "ts" => (include_str!("../assets/typescript.txt"), |s| s.blue().bold()),
        "python" => (include_str!("../assets/python.txt"), |s| s.yellow().bold()),
        "javascript" | "js" => (include_str!("../assets/javascript.txt"), |s| s.yellow().bold()),
        _ => (include_str!("../assets/default.txt"), |s| s.white().bold()),
    };

    let max_width = raw_logo
        .lines()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);

    let logo_lines: Vec<ColoredString> = raw_logo
        .lines()
        .map(|line| {
            let char_count = line.chars().count();
            let padding = " ".repeat(max_width.saturating_sub(char_count));
            color_func(&format!("{line}{padding}"))
        })
        .collect();

    (logo_lines, max_width, color_func)
}