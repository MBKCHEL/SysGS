use git2::{ObjectType, TreeWalkMode, TreeWalkResult};
use std::collections::HashMap;
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
    pub repo_name: String,
    pub age: String,
}

fn is_ignored_language(lang: &LanguageType) -> bool {
    matches!(
        lang,
        LanguageType::Toml
            | LanguageType::Xml
            | LanguageType::Json
            | LanguageType::Yaml
            | LanguageType::Markdown
            | LanguageType::Text
    )
}

fn normalize_language_type(lang: LanguageType) -> LanguageType {
    match lang {
        LanguageType::CHeader => LanguageType::C,
        LanguageType::CppHeader => LanguageType::Cpp,
        other => other,
    }
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
    let mut author_counts: HashMap<String, usize> = HashMap::new();
    let mut total_commits = 0;
    let mut first_commit_time: Option<i64> = None;

    for oid in revwalk.flatten() {
        total_commits += 1;
        if let Ok(c) = repo.find_commit(oid) {
            first_commit_time = Some(c.time().seconds());

            let author = c.author();
            if let Ok(name) = author.name() {
                *author_counts.entry(name.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    let age = if let Some(first_time) = first_commit_time {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let diff_sec = now - first_time;
        let days = diff_sec / 86400;

        if days < 1 {
            "today".to_string()
        } else if days < 30 {
            format!("{} days", days)
        } else if days < 365 {
            let months = days / 30;
            format!("{} months", months)
        } else {
            let years = days / 365;
            let remaining_months = (days % 365) / 30;
            if remaining_months > 0 {
                format!("{} years, {} months", years, remaining_months)
            } else {
                format!("{} years", years)
            }
        }
    } else {
        "unknown".to_string()
    };

    fn get_repo_name(repo: &git2::Repository) -> Option<String> {
        repo.path()
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
    }

    let repo_name = get_repo_name(repo).unwrap_or_else(|| "Unknown Repo".to_string());

    let mut language_stats: HashMap<LanguageType, CodeStats> = HashMap::new();

    head_tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        if entry.kind() == Some(ObjectType::Blob) {
            if let Ok(name) = std::str::from_utf8(entry.name_bytes()) {
                let full_path = std::path::Path::new(root).join(name);
                if let Some(raw_lang_type) = LanguageType::from_path(&full_path, &config) {
                    let lang_type = normalize_language_type(raw_lang_type);

                    if !is_ignored_language(&lang_type) {
                        if let Ok(object) = entry.to_object(&repo) {
                            if let Some(blob) = object.as_blob() {
                                if !blob.is_binary() {
                                    let stats = lang_type.parse_from_slice(blob.content(), &config);

                                    total_files += 1;

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

    let total_lines: usize = language_stats.values().map(|stats| stats.code).sum();

    let mut sorted_langs: Vec<(&LanguageType, &CodeStats)> = language_stats.iter().collect();
    sorted_langs.sort_by(|a, b| b.1.code.cmp(&a.1.code));

    Ok(RepoStats {
        id,
        branch: branch.to_string(),
        authors_str,
        repo_name,
        total_files: total_files.to_string(),
        total_code_lines: total_lines,
        total_commits,
        sorted_langs: sorted_langs
            .into_iter()
            .map(|(lang, stats)| (format!("{:?}", lang), stats.code))
            .collect(),
        summary: summary.to_string(),
        age,
    })
}