use colored::*;

pub fn get_logo(
    lang: &str,
) -> (
    Vec<ColoredString>,
    usize,
    Box<dyn Fn(&str) -> ColoredString>,
) {
    let lang_lower = lang.to_lowercase();

    let (raw_logo, color_func): (&str, Box<dyn Fn(&str) -> ColoredString>) =
        match lang_lower.as_str() {
            "rust" => (
                include_str!("../assets/rust.txt"),
                Box::new(|s| s.white().bold()),
            ),
            "zsh" => (
                include_str!("../assets/zsh.txt"),
                Box::new(|s| s.white().bold()),
            ),
            "bash" => (
                include_str!("../assets/bash.txt"),
                Box::new(|s| s.white().bold()),
            ),
            "markdown" | "md" => (
                include_str!("../assets/markdown.txt"),
                Box::new(|s| s.white().bold()),
            ),
            "c" => (
                include_str!("../assets/c.txt"),
                Box::new(|s| s.blue().bold()),
            ),
            "go" | "golang" => (
                include_str!("../assets/go.txt"),
                Box::new(|s| s.blue().bold()),
            ),
            "c++" | "cpp" => (
                include_str!("../assets/cpp.txt"),
                Box::new(|s| s.blue().bold()),
            ),
            "lua" | "lualeng" => (
                include_str!("../assets/lua.txt"),
                Box::new(|s| s.blue().bold()),
            ),
            "asm" | "assembly" | "assemblygas" => (
                include_str!("../assets/asm.txt"),
                Box::new(|s| s.magenta().bold()),
            ),
            "c#" | "csharp" => (
                include_str!("../assets/csharp.txt"),
                Box::new(|s| s.purple().bold()),
            ),
            "php" => (
                include_str!("../assets/php.txt"),
                Box::new(|s| s.purple().bold()),
            ),
            "java" => (
                include_str!("../assets/java.txt"),
                Box::new(|s| s.red().bold()),
            ),
            "swift" | "swiftleng" => (
                include_str!("../assets/swift.txt"),
                Box::new(|s| s.red().bold()),
            ),
            "ruby" => (
                include_str!("../assets/ruby.txt"),
                Box::new(|s| s.red().bold()),
            ),
            "typescript" | "ts" => (
                include_str!("../assets/typescript.txt"),
                Box::new(|s| s.blue().bold()),
            ),
            "python" | "py" => (
                include_str!("../assets/python.txt"),
                Box::new(|s| s.blue().bold()),
            ),
            "javascript" | "js" => (
                include_str!("../assets/javascript.txt"),
                Box::new(|s| s.yellow().bold()),
            ),
            "kotlin" => (
                include_str!("../assets/kotlin.txt"),
                Box::new(|s| {
                    let chars: Vec<char> = s.chars().collect();
                    let len = chars.len();
                    let mut graduated = String::new();

                    for (i, ch) in chars.iter().enumerate() {
                        let t = if len > 1 {
                            i as f32 / (len - 1) as f32
                        } else {
                            0.0
                        };
                        let r = (127.0 + (248.0 - 127.0) * t) as u8;
                        let g = (82.0 + (143.0 - 82.0) * t) as u8;
                        let b = (199.0 + (30.0 - 199.0) * t) as u8;

                        graduated.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch));
                    }
                    graduated.into()
                }),
            ),
            _ => (
                include_str!("../assets/default.txt"),
                Box::new(|s| s.white().bold()),
            ),
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
