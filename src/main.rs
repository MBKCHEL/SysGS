use git2::{Repository};
mod info;

fn main() -> Result<(), git2::Error> {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(_) => {
            eprintln!("Error!!! In your Project not found git!.");
            std::process::exit(1);
        }
    };

    let stats = info::get_info(&repo)?;

    println!("Head: {} ({})", stats.id, stats.branch);
    println!("Authors: {}", stats.authors_str);
    println!("Files: {}", stats.total_files);
    println!("Lines of code: {}", stats.total_code_lines);
    print!("Languages: ");
    let mut first = true;
    for (lang, lang_code_lines) in &stats.sorted_langs {
        let percentage: f64 = if stats.total_code_lines > 0 {
            (*lang_code_lines as f64 / stats.total_code_lines as f64) * 100.0
        } else {
            0.0
        };

        if !first {
            print!(", ");
        }
        print!("{} ({:.1}%)", lang, percentage);
        first = false;
    }
    println!("\nTotal commits: {}", stats.total_commits);
    println!("Last commit: {}", stats.summary);

    Ok(())
}
