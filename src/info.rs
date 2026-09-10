use std::collections::HashMap;
use git2::{ObjectType, TreeWalkMode, TreeWalkResult};
use tokei::{CodeStats, Config, LanguageType};

pub struct RepoStats {
    pub id: String,
    pub branch: String,
    pub sorted_langs: Vec<(String, usize)>,
    pub authors_str: String,
    pub total_files: String,
    pub total_code_lines: usize,
    pub total_commits: i32,
    pub summary: String,
}

pub fn get_info(repo: &git2::Repository) -> Result<RepoStats, git2::Error> {
    let head = repo.head()?;
    let head_tree = head.peel_to_tree()?;
    let commit = head.peel_to_commit()?;
    let summary = commit.summary()?.unwrap_or("default");
    let id = commit.id();
    let id = format!("{:.7}", id);

    let branch = head.shorthand().unwrap_or("HEAD");

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

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

    let top_names: Vec<String> = sorted_authors
        .iter()
        .take(5)
        .map(|(name, _)| name.clone())
        .collect();

    let authors_str = if sorted_authors.len() > 5 {
        format!("{} (total {})", top_names.join(", "), sorted_authors.len())
    } else {
        top_names.join(", ")
    };

    let total_lines: usize = language_stats
        .iter()
        .filter(|(lang, _)| {
            !matches!(
                lang,
                LanguageType::Toml
                    | LanguageType::Xml
                    | LanguageType::Json
                    | LanguageType::Yaml
                    | LanguageType::Markdown
                    | LanguageType::Text
            )
        })
        .map(|(_, stats)| stats.code)
        .sum();

    let mut sorted_langs: Vec<(&LanguageType, &CodeStats)> = language_stats
        .iter()
        .filter(|(lang, _)| {
            !matches!(
                lang,
                LanguageType::Toml
                    | LanguageType::Xml
                    | LanguageType::Json
                    | LanguageType::Yaml
                    | LanguageType::Markdown
                    | LanguageType::Text
            )
        })
        .collect();

    sorted_langs.sort_by(|a, b| b.1.code.cmp(&a.1.code));
    Ok(RepoStats {
        id,
        branch: branch.to_string(),
        authors_str,
        total_files: total_files.to_string(),
        total_code_lines: total_lines, 
        total_commits,
        sorted_langs: sorted_langs
            .into_iter()
            .map(|(lang, stats)| (format!("{:?}", lang), stats.code))
            .collect(),
        summary: summary.to_string(),
    })
}