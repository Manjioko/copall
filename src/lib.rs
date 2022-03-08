use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn cp_a_dir_file(mut args: env::Args) -> Result<(), Box<dyn Error>> {
    // 第一个参数是程序的名称,不需要
    args.next();

    //获取第一个参数
    let orgin_dir_or_file = match args.next() {
        Some(g) => g,
        None => return Err("参数错误: 希望有'2'个参数,但得到'0'个".into()),
    };

    //获取第二个参数
    let dest_dir = match args.next() {
        Some(g) => g,
        None => return Err("参数错误: 希望有'2'个参数,但得到'1'个".into()),
    };

    // 判断第一个参数时候是文件
    let is_file = Path::new(&orgin_dir_or_file).is_file();

    // 判断第一个参数是否是文件夹
    let is_dir = Path::new(&orgin_dir_or_file).is_dir();

    // 判断第一个参数是否存在
    let is_exist_origin = Path::new(&orgin_dir_or_file).exists();

    //判断第二个参数是否存在文件夹
    let such_dir_exist = Path::new(&dest_dir).exists();

    // 判断第二个参数是否是文件
    let dir_not_file = Path::new(&dest_dir).is_file();

    // 如果第二个参数是文件,返回错误信息
    if dir_not_file {
        let filename = "第二个参数应该为一个文件夹,但找到一个文件: ".to_string() + &dest_dir;
        return Err(filename.into());
    }

    // 查看要拷贝的文件是否存在
    if !is_exist_origin && !is_dir {
        let err_msg = "文件 ".to_string() + &orgin_dir_or_file + " 不存在";
        return Err(err_msg.as_str().into());
    }

    //如果文件夹不存在,就新创建一个
    if !such_dir_exist {
        fs::create_dir_all(&dest_dir)?;
    }

    // 第一个参数是文件时执行
    if is_file {
        let mut newpath = PathBuf::from(&dest_dir);
        let filname = Path::new(&orgin_dir_or_file).file_name().unwrap();
        newpath.push(filname);
        println!(
            ">>> START COPING >>> {:?}, >>> TO >>> {:?}",
            &orgin_dir_or_file, &newpath
        );
        fs::copy(&orgin_dir_or_file, &newpath)?;
    }

    //第一个参数是文件夹时执行
    if is_dir {
        let dest_dir_name = PathBuf::from(&dest_dir);
        let dir_path = Path::new(&orgin_dir_or_file);
        recursion_dir(dir_path, dest_dir_name.as_os_str())?;
    }

    Ok(())
}

fn recursion_dir(dir_path: &Path, dest_dir_name: &OsStr) -> Result<(), Box<dyn Error>> {

    // 读取文件目录
    let suc_to_read_dir = fs::read_dir(dir_path)?;

    // 遍历文件目录
    for read_dir in suc_to_read_dir {

        //读取文件路径
        let real_read_dir = read_dir?.path();

        // 是文件夹就进入递归
        if Path::new(&real_read_dir).is_dir() {

            // 拼接文件路径
            let mut dest_dir_name = PathBuf::from(dest_dir_name);
            let filename = Path::new(&real_read_dir).file_name().unwrap();
            dest_dir_name.push(filename);

            // 目的地路径如果不存在,就创建一个新路径
            if !dest_dir_name.exists() {
                fs::create_dir_all(&dest_dir_name)?;
            }

            // 递归
            recursion_dir(Path::new(&real_read_dir), dest_dir_name.as_os_str())?;
        }

        // 是文件就拷贝文件
        if Path::new(&real_read_dir).is_file() {

            // 将 &OsStr 转换成PathBuf,更新保存路径 拼接文件路径
            let mut dest_dir_name = PathBuf::from(dest_dir_name);
            let filename = Path::new(&real_read_dir).file_name().unwrap();
            dest_dir_name.push(filename);

            // from_file_name 要拷贝的文件名称  to_file_name 文件要拷贝到目的地的文件名称
            let from_file_name = Path::new(&real_read_dir);
            let to_file_name = Path::new(&dest_dir_name);
            println!(
                ">>> START COPING >>> {:?}, >>> TO >>> {:?}",
                from_file_name, to_file_name
            );

            // 拷贝核心代码
            fs::copy(from_file_name, to_file_name)?;
        }
    }
    Ok(())
}
