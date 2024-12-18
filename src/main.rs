use clap::Parser;
use git2::{Repository, TreeWalkMode};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    /// Path to the repository
    #[arg(short, long)]
    repo: String,

    /// Output file
    #[arg(short, long, default_value = "repo_contents.txt")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Open the repository
    let repo = Repository::open(&args.repo)?;
    let tree = repo.head()?.peel_to_tree()?;

    // Open the output file
    let mut output_file = BufWriter::new(File::create(&args.output)?);

    // Walk through the repository tree
    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        let full_path = Path::new(root).join(entry.name().unwrap_or_default());
        if entry.kind() == Some(git2::ObjectType::Blob) {
            if let Err(e) = append_file_content(&repo, &full_path.to_string_lossy(), &mut output_file) {
                eprintln!("Failed to process {}: {}", full_path.to_string_lossy(), e);
            }
        }

        git2::TreeWalkResult::Ok
    })?;

    println!("Contents written to {}", args.output);
    Ok(())
}

fn append_file_content(
    repo: &Repository,
    file_path: &str,
    output: &mut BufWriter<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    let obj = repo.revparse_single("HEAD")?; // Get the HEAD commit
    let tree = obj.peel_to_tree()?; // Get the tree from HEAD

    // Use tree to find the specific file
    if let Some(entry) = tree.get_path(Path::new(file_path)).ok() {
        if let Ok(blob) = entry.to_object(repo).and_then(|obj| obj.peel_to_blob()) {
            writeln!(output, "\n\n---")?;
            writeln!(output, "FILE_PATH: {}", file_path)?;
            writeln!(output, "```")?;
            writeln!(output, "{}", String::from_utf8_lossy(blob.content()))?;
            writeln!(output, "```")?;
        } else {
            eprintln!("Failed to resolve blob for file: {}", file_path);
        }
    } else {
        eprintln!("File not found in tree: {}", file_path);
    }

    Ok(())
}
