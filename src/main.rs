use std::env;
use std::fs;
use std::path::{Path, PathBuf};
fn find_dir<P: AsRef<Path>>(current_dir: P, dirname: &str, ignore_list: &[&str]) -> Option<PathBuf> {
    let entries = match fs::read_dir(current_dir) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    for entry in entries {
        println!("entry: {:?}", entry);
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
        } else if metadata.is_dir() {
            let entry_name = match entry.file_name().into_string() {
                Ok(name) => name,
                Err(_) => continue,
            };

            // Skip directories in the ignore list
            if ignore_list.contains(&entry_name.as_str()) {
                continue;
            }

            if entry_name == dirname {
                return Some(entry.path());
            } else {
                if let Some(found_directory) = find_dir(entry.path(), dirname, ignore_list) {
                    return Some(found_directory);
                }
            }
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
    let root_dir = Path::new("/"); // For Unix/Linux. On Windows, you might use "C:\\"
    let ignore_list = [
        "node_modules", // Common in Node.js projects
        ".git",         // Git directory
        "vendor",       // Common in various language package managers
        "build",        // Common build output directory
        "dist",         // Distribution directory
        ".idea",        // JetBrains IDE config folder
        "__pycache__",  // Python cache directory
        "target",       // Rust build directory
        "bin",          // Binary output directory
        "obj",          // Object file directory
        "log",          // Log directory
        // Add more directories as needed
    ];

    if let Some(found_directory) = find_dir(&root_dir, dirname, &ignore_list) {
        println!("{}", found_directory.display());
    } else {
        println!("Directory not found");
    }

    Ok(())

}
