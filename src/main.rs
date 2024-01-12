use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use std::collections::VecDeque;

fn find_dir<P: AsRef<Path>>(current_dir: P, dirname: &str, ignore_list: &[&str]) -> Option<PathBuf> {
    let mut queue = VecDeque::new();
    queue.push_back(current_dir.as_ref().to_path_buf());

    while let Some(path) = queue.pop_front() {
        let entries = match fs::read_dir(&path) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        println!("path: {}", path.display());

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };

            if metadata.is_file() {
                continue;
            }

            let entry_name = match entry.file_name().into_string() {
                Ok(name) => name,
                Err(_) => continue,
            };
            println!("entry_name: {}", entry_name);


            if ignore_list.contains(&entry_name.as_str()) {
                continue;
            }

            if entry_name == dirname {
                println!("found it");
                return Some(entry.path());
            }

            queue.push_back(entry.path());
        }
    }

    None
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: goto <dirname>");
        return Ok(());
    }

    let dirname = &args[1];
    let root_dir = Path::new("/mnt/c/Users/Shipt/desktop");
    let ignore_list = [
        "node_modules", // Common in Node.js projects
        ".git",         // Git directory
        "vendor",       // Common in various language package managers
        "build",        // Common build output directory
        "dist",         // Distribution directory
        "__pycache__",  // Python cache directory
        "target",       // Rust build directory
        "bin",          // Binary output directory
        "obj",          // Object file directory
        "log",
        ".vscode",
        "AppData",
        "venv"        // Log directory
        // Add more directories as needed
    ];

    if let Some(found_directory) = find_dir(&root_dir, dirname, &ignore_list) {
        println!("{}", found_directory.display());
    } else {
        println!("Directory not found");
    }

    Ok(())

}
