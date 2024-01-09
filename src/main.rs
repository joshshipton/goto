use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn find_dir<P: AsRef<Path>>(current_dir: P, dirname: &str) -> Option<PathBuf> {
    let entries = match fs::read_dir(current_dir) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    for entry in entries {
        println!("entry is {:?}", entry);
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
            if let Some(entry_name) = entry.file_name().to_str() {
                if entry_name == dirname {
                    return Some(entry.path());
                } else {
                    if let Some(found_directory) = find_dir(entry.path(), dirname) {
                        return Some(found_directory);
                    }
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

    if let Some(found_directory) = find_dir(&root_dir, dirname) {
        println!("cd {}", found_directory.display());
    } else {
        println!("Directory not found");
    }

    Ok(())
}
