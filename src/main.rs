use std::env;
use std::fs;
use std::io;
use std::io::Error;
use std::path::{Path, PathBuf};
// use std::thread;

fn main() {
    // let p = PathBuf::from(".");
    // let mut sn = String::new();
    let mut args = Vec::new();
    // match io::stdin().read_line(&mut sn) {
    //     Ok(_) => {
    //         println!("reading...");
    //         let str_in = sn.trim().to_string();
    //         match find_file(p,&str_in) {
    //             Ok(s) => println!("{}",s),
    //             Err(err) => println!("Error: {:?}", err)
    //         }
    //     },
    //     Err(e) => {
    //         println!("{}",e);
    //     }
    // };

    for arg in env::args() {
        args.push(arg);
    }

    // let res = parser_args(args);
    match parser_args(args) {
        Ok(_) => (),
        Err(e) => println!("error:{}", e),
    }
    // println!("{:?}",args);
}

fn parser_args(args: Vec<String>) -> Result<String, Error> {
    if args.len() > 1 {
        match args[1].trim() {
            "-r" => {
                // println!("hell owlrd");
                if args.len() >= 3 {
                    let path = PathBuf::from(args[2].trim());
                    let str_ser = &args[1];
                    match find_file(path, &str_ser) {
                        Ok(_) =>(),
                        Err(err) => println!("Error: {:?}", err),
                    }
                } else {
                    println!("参数不全");
                }

                return Ok("-r".to_string());
            }
            "-d" => {
                // println!("fuck world");
                return Ok("-d".to_string());
            }
            _ => {
                // 起始路径
                let path = PathBuf::from(".");
                let str_ser = &args[1];
                match find_file(path, &str_ser) {
                    Ok(s) => println!("{}", s),
                    Err(err) => println!("Error: {:?}", err),
                }
                return Ok("#".to_string());
            }
        }
    } else {
        Ok("*".to_string())
    }
}

fn find_file(path: PathBuf, query: &String) -> io::Result<&str> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let strpath = entry.path();
                // strpath.push("");
                if Path::new(&strpath).is_dir() {
                    find_file(strpath, query)?;
                } else {
                    if !Path::new("DIR").exists() {
                        fs::create_dir("DIR")?;
                    }
                    let np = entry.path();
                    let mut or = PathBuf::from("DIR");
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
    Ok("Success!")
}
