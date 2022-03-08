use cpa;
use std::env;

fn main() {
    // env::args() 返回一个遍历器,作为参数传入cp_a_dir_file
    if let Err(e) = cpa::cp_a_dir_file(env::args()) {
        eprintln!(">>>>> {}", e)
    }
}
