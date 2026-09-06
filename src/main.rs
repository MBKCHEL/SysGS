use git2::{Repository, TreeWalkMode, TreeWalkResult, ObjectType};
use tokei::{Config, LanguageType, CodeStats, Languages};
use std::collections::HashMap;
fn main() -> Result<(), git2::Error> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(_) => {
            eprintln!("Error!!! In your Project not found git!.");
            std::process::exit(1);
        }
    };

    let head = repo.head()?;
    let head_tree = repo.head()?.peel_to_tree()?;
    let commit = head.peel_to_commit()?;
    let summary = commit.summary()?.unwrap_or("default");
    let id = commit.id();
    let id = format!("{:.7}", id);

    let branch = head.shorthand().unwrap_or("HEAD");

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    

    let mut languages = Languages::new();
    let config = Config::default();

    let mut total_files = 0;
    let mut total_code_lines = 0;

    let mut author_counts: HashMap<String, usize> = HashMap::new();

    let mut total_commits = 0;

    for oid in revwalk.flatten() {
        total_commits += 1;
        if let Ok(c) = repo.find_commit(oid) {
            let author = c.author();
            if author.name().is_ok() {
                let name = author.name().expect("ERROR!!!!").to_string();
                *author_counts.entry(name).or_insert(0) += 1;
            }
        }
    }

    let mut language_stats: HashMap<LanguageType, CodeStats> = HashMap::new();

    head_tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        if entry.kind() == Some(ObjectType::Blob) {
            if let Ok(name) = std::str::from_utf8(entry.name_bytes()) {
                let full_path = std::path::Path::new(root).join(name);
                if let Some(lang_type) = LanguageType::from_path(&full_path, &config) {
                    if let Ok(object) = entry.to_object(&repo) {
                        if let Some(blob) = object.as_blob() {
                            if !blob.is_binary() {
                                let stats = lang_type.parse_from_slice(blob.content(), &config);

                                total_files += 1;
                                total_code_lines += stats.code;
                                let lang_stats = language_stats
                                .entry(lang_type)
                                .or_insert_with(CodeStats::new);
                                *lang_stats += stats;
                            }
                        }
                    }
                }
            }
        }
        TreeWalkResult::Ok
    })?;


    let mut sorted_authors: Vec<(String, usize)> = author_counts.into_iter().collect();
    sorted_authors.sort_by(|a, b| b.1.cmp(&a.1));

    let top_names: Vec<String> = sorted_authors.iter().take(5).map(|(name, _)| name.clone()).collect();

    let authors_str = if sorted_authors.len() > 5 {
        format!(
            "{} (total {})",
            top_names.join(", "),
            sorted_authors.len()
        )
    } else {
        top_names.join(", ")
    };

    let total_lines: usize = language_stats
        .iter()
        .filter(|(lang, _)| {
            !matches!(
                lang,
                tokei::LanguageType::Toml
                    | tokei::LanguageType::Xml
                    | tokei::LanguageType::Json
                    | tokei::LanguageType::Yaml
                    | tokei::LanguageType::Markdown
            )
        })
        .map(|(_, stats)| stats.code)
        .sum();

    let mut sorted_langs: Vec<(&tokei::LanguageType, &tokei::CodeStats)> = language_stats
        .iter()
        .filter(|(lang, _)| {
            !matches!(
                lang,
                tokei::LanguageType::Toml
                    | tokei::LanguageType::Xml
                    | tokei::LanguageType::Json
                    | tokei::LanguageType::Yaml
                    | tokei::LanguageType::Markdown
            )
        })
        .collect();
        
    sorted_langs.sort_by(|a, b| b.1.code.cmp(&a.1.code));

    println!("Head: {} ({})", id, branch);
    println!("Authors: {}", authors_str);
    println!("Files: {}", total_files);
    println!("Lines of code: {}", total_code_lines);
    print!("Languages: ");
    let mut first = true;
    for (lang, stats) in sorted_langs {
        let percentage = if total_lines > 0 {
            (stats.code as f64 / total_lines as f64) * 100.0
        } else {
            0.0
        };

        if !first {
            print!(", ");
        }
        print!("{:?} ({:.1}%)", lang, percentage);
        first = false;
    }
    println!("\nTotal commits: {}", total_commits);
    println!("Last commit: {}", summary);


    Ok(())
}
