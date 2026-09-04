use git2::Repository;

fn main() -> Result<(), git2::Error> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("failes to init {e}"),
    };

    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    let id = commit.id();
    let id = format!("{:.7}", id);

    let branch = head.shorthand().unwrap_or("HEAD");

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut author_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for oid in revwalk.flatten() {
        if let Ok(c) = repo.find_commit(oid) {
            let author = c.author();
            if author.name().is_ok() {
                let name = author.name().unwrap().to_string();
                *author_counts.entry(name).or_insert(0) += 1;
            }
        }
    }

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

    println!("Authors: {}", authors_str);
    println!("Head: {} ({})", id, branch);
    println!("Last commit: {}", commit.message().unwrap_or(""));

    Ok(())
}
