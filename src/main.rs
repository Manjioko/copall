// use std::process::Command;
use std::fs;
use std::io;
use std::path::{Path,PathBuf};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let p = PathBuf::from(".");
    match args.get(1){
        Some(str) => {
            match find_file(p,&str) {
                Ok(s) => println!("Success: {:?}",s),
                Err(err) => println!("Error: {:?}", err)
            }
        },
        None => {
            let s = String::from("*");
            match find_file(p,&s) {
                Ok(s) => println!("Success: {:?}",s),
                Err(err) => println!("Error: {:?}", err)
            }
        }
    } 
}


fn find_file(path: PathBuf,query: &String) -> io::Result<()> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let strpath = entry.path();
                // strpath.push("");
                if Path::new(&strpath).is_dir() {
                    find_file(strpath,query)?;
                } else {
                    if !Path::new("newDIR").exists() {
                        fs::create_dir("newDIR")?;
                    }
                    let np = entry.path();
                    let mut or = PathBuf::from("newDIR");
                    or.push(entry.file_name());
                    if query != "*" {
                        if entry.file_name().to_string_lossy().contains(query) {
                            fs::copy(np, or)?;
                        }
                    } else {
                        fs::copy(np, or)?;
                    }
                    
                }
            }
        }
    }
    Ok(())
}