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

    let authors = commit.author();
    let authors = authors.name().unwrap_or("Unknown");

    let branch = head.shorthand().unwrap_or("HEAD");

    println!("Authors: {}", authors);
    println!("Head: {} ({})", id, branch);
    println!("Last commit: {}", commit.message().unwrap_or(""));

    Ok(())
}
