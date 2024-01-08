use std::env;
use std::fs;
use std::path::Path

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 
    {
        eprintln!("dumbass its: goto <dirname>");
        return;
    }
    // the name of the directory
    let dirname = &args[1];
    let current_dir = env::current_dir()?;

    for element in fs::read_dir(current_dir)?
    {

        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {continue;}
        else if metadata.is_dir(){
            if let Some(entry_name) = entry.file_name().to_str() {
                if entry_name == dirname {
                    println!("cd {}", found_directory.display());
                    break; 
                }
            }

        }  
        

    }           
    }
    

