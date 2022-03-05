use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::env;

pub fn parser_args(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() > 1 {
        match args[1].trim() {
            "-r" => {
                if args.len() >= 3 {
                    let path = PathBuf::from(args[2].trim());
                    let str_ser = &args[1];
                    find_file(path, &str_ser)?;
                    return Ok(());
                }
                Err("参数不全".into())
            }
            "-d" => {
                return Ok(());
            }
            _ => {
                // 起始路径
                let path = PathBuf::from(".");
                let str_ser = &args[1];
                if let Err(err) = find_file(path, &str_ser) {
                    println!("Error: {:?}", err);
                    process::exit(1);
                }
                return Ok(());
            }
        }
    } else {
        Ok(())
    }
}

pub fn parser_args_new(mut args: env::Args) -> Result<(), Box<dyn Error>> {
    args.next();

    // 第一个参数
    let sec = match args.next() {
        Some(sec_xx) => sec_xx,
        None => return Err("参数缺失: 程序希望有这2个参数,但是找到0个参数".into()),
    };

    // 目的地址
    let thr = match args.next() {
        Some(thr) => thr,
        None => return Err("参数缺失: 程序希望有这2个参数,但是找到1个参数".into()),
    };

    let sec_str = sec.as_str();
    let thr_str = thr.as_str();
    match sec_str {
        "-r" => {},
        "-f" => {},
        "."  => {},
        _    => {},
    } 

    Ok(())
}

fn find_file(path: PathBuf, query: &String) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
