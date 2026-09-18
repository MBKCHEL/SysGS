use colored::*;

enum LogoStyle {
    Solid(Color),
    KotlinGradient,
}

impl LogoStyle {
    fn apply(&self, s: &str) -> ColoredString {
        match self {
            LogoStyle::Solid(c) => s.color(*c).bold(),
            LogoStyle::KotlinGradient => kotlin_gradient(s).into(),
        }
    }
}

fn kotlin_gradient(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut out = String::with_capacity(s.len() * 4);

    for (i, ch) in chars.iter().enumerate() {
        let t = if len > 1 {
            i as f32 / (len - 1) as f32
        } else {
            0.0
        };
        let r = (127.0 + (248.0 - 127.0) * t) as u8;
        let g = (82.0 + (143.0 - 82.0) * t) as u8;
        let b = (199.0 + (30.0 - 199.0) * t) as u8;
        out.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}\x1b[0m"));
    }
    out
}

pub fn get_logo(lang: &str) -> (Vec<ColoredString>, usize) {
    let lang_lower = lang.to_lowercase();

    let (raw_logo, style): (&str, LogoStyle) = match lang_lower.as_str() {
        "rust" => (
            include_str!("../assets/rust.txt"),
            LogoStyle::Solid(Color::White),
        ),
        "zsh" => (
            include_str!("../assets/zsh.txt"),
            LogoStyle::Solid(Color::White),
        ),
        "bash" => (
            include_str!("../assets/bash.txt"),
            LogoStyle::Solid(Color::White),
        ),
        "markdown" | "md" => (
            include_str!("../assets/markdown.txt"),
            LogoStyle::Solid(Color::White),
        ),
        "c" => (
            include_str!("../assets/c.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "go" | "golang" => (
            include_str!("../assets/go.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "c++" | "cpp" => (
            include_str!("../assets/cpp.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "lua" | "lualeng" => (
            include_str!("../assets/lua.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "asm" | "assembly" | "assemblygas" => (
            include_str!("../assets/asm.txt"),
            LogoStyle::Solid(Color::Magenta),
        ),
        "c#" | "csharp" => (
            include_str!("../assets/csharp.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "php" => (
            include_str!("../assets/php.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "java" => (
            include_str!("../assets/java.txt"),
            LogoStyle::Solid(Color::Red),
        ),
        "swift" | "swiftleng" => (
            include_str!("../assets/swift.txt"),
            LogoStyle::Solid(Color::Red),
        ),
        "ruby" => (
            include_str!("../assets/ruby.txt"),
            LogoStyle::Solid(Color::Red),
        ),
        "typescript" | "ts" => (
            include_str!("../assets/typescript.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "python" | "py" => (
            include_str!("../assets/python.txt"),
            LogoStyle::Solid(Color::Blue),
        ),
        "javascript" | "js" => (
            include_str!("../assets/javascript.txt"),
            LogoStyle::Solid(Color::Yellow),
        ),
        "kotlin" => (
            include_str!("../assets/kotlin.txt"),
            LogoStyle::KotlinGradient,
        ),
        _ => (
            include_str!("../assets/default.txt"),
            LogoStyle::Solid(Color::White),
        ),
    };

    let max_width = raw_logo
        .lines()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);

    let logo_lines = raw_logo
        .lines()
        .map(|line| {
            let pad = max_width - line.chars().count();
            let padded = format!("{line}{}", " ".repeat(pad));
            style.apply(&padded)
        })
        .collect();

    (logo_lines, max_width)
}
