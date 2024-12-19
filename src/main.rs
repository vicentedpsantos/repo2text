use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the root of the Git repository
    #[clap(short, long, value_parser)]
    repo: PathBuf,

    /// Path to the output .txt file
    #[clap(short, long, value_parser)]
    output: PathBuf,

    /// Directories to exclude from processing
    #[clap(long, value_parser)]
    excluding: Vec<PathBuf>,
}

fn is_binary_file(path: &Path) -> bool {
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 512];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            return buffer[..bytes_read].iter().any(|&byte| byte == 0);
        }
    }
    false
}

fn is_image_file(path: &Path) -> bool {
    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        let lower_ext = extension.to_lowercase();
        matches!(
            lower_ext.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" | "svg"
        )
    } else {
        false
    }
}

fn should_skip_file(path: &Path) -> bool {
    is_binary_file(path) || is_image_file(path)
}

fn process_repository(repo_path: &Path, output_file: &mut File, repo_name: &str, excluding: &[PathBuf]) {
    for entry in fs::read_dir(repo_path).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();

            if excluding.iter().any(|exclude| path.starts_with(exclude)) {
                println!("Excluding path: {:?}", path);
                continue;
            }

            if path.is_file() {
                if should_skip_file(&path) {
                    println!("Skipping file: {:?}", path);
                } else {
                    println!("Processing file: {:?}", path);
                    if let Ok(contents) = fs::read_to_string(&path) {
                        let relative_path = path.strip_prefix(repo_path).unwrap_or(&path);
                        let full_relative_path = format!("{}/{}", repo_name, relative_path.display());
                        writeln!(
                            output_file,
                            "---\nFILE_PATH: {}\n```\n{} ```\n",
                            full_relative_path,
                            contents
                        )
                        .expect("Failed to write to output file");
                    } else {
                        eprintln!("Failed to read file: {:?}", path);
                    }
                }
            } else if path.is_dir() {
                process_repository(&path, output_file, repo_name, excluding);
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let repo_name = args.repo.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("repository");

    let mut output_file = File::create(&args.output).expect("Failed to create output file");

    process_repository(&args.repo, &mut output_file, repo_name, &args.excluding);

    println!("Repository processing completed. Output saved to {:?}", args.output);
}
