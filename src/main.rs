extern crate pychecker;
use pychecker::io_tools::{read_str, get_py_files, correct_path};
use pychecker::checker::*;
use std::env::args;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        println!("Usage: $ pychecker file.table");
        return;
    }

    let dirfile = read_str(&args[1]);
    let dirs_to_check: Vec<&str> = dirfile.split("\n").collect();
    for dir in dirs_to_check {
        let pyfiles = get_py_files(dir);
        for file in &pyfiles {
            let log = check_ios(&correct_path(&format!("{}", file)));
            if log.len() > 0 {
                println!("{}", log.join("\n"));
            }
        }
    }

}
