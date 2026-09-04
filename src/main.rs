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
        if let Some(ObjectType::Blob) = entry.kind() {
            if let (Some(name), Ok(object)) = (entry.name().ok(), entry.to_object(&repo)) {
                if let Some(blob) = object.as_blob() {
                    if !blob.is_binary() {
                        if let Some(lang_type) = LanguageType::from_path(name, &config) {
                            let mut stats = lang_type.parse_from_slice(blob.content(), &config);

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

    println!("Head: {} ({})", id, branch);
    println!("Authors: {}", authors_str);
    println!("Files: {}", total_files);
    println!("Lines of code: {}", total_code_lines);
    println!("Total commits: {}", total_commits);
    println!("Last commit: {}", summary);


    Ok(())
}
