use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::fs::{File, read_dir};
use std::path::Path;

///  Contains some useful functions for reading/writing to std/file


/// Reads filename and returns vector of bytes
/// 
/// # Examples
/// 
/// ```rust
/// let file_content = read_bytes("path/to/file");
/// ```
pub fn read_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer: Vec<u8> = vec![];
    f.read_to_end(&mut buffer).expect("Couldn`t read to string");
    buffer
}


/// Reads filename and returns String
/// 
/// Reads filename and returns String, with replaced CRLF to LF
/// 
/// # Examples
/// 
/// ```rust
/// let file_string = read_str("path/to/file");
/// ```
pub fn read_str(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Couldn`t read to string");
    String::from(buffer.replace("\r\n", "\n").trim())
}

#[cfg(target_os = "windows")]
fn joiner() -> &'static str {
    "\\"
}

#[cfg(not(target_os = "windows"))]
fn joiner() -> &'static str {
    "/"
}

/// Corrects your Unix path on various platforms
///
/// Returns `path\to\file` on Windows and `path/to/file` on Unix
/// 
/// # Examples
/// 
/// ```rust
/// assert_eq!(String::from("path/to/file"), correct_path("path/to/file")); // on Unix
/// assert_eq!(String::from("path\\to\\file"), correct_path("path/to/file")); // on Windows
/// ```
pub fn correct_path(path: &str) -> String {
    path.replace("/", joiner())
}


/// Writes String to your file
/// 
/// # Examples
///
/// ```rust
/// write_to_file("/path/to/file", "I`m file");
/// ```
pub fn write_to_file(path: &str, content: String) {
    let mut file = File::create(path).unwrap();
    file.write_fmt(format_args!("{}", content)).expect("Couldn`t write to file");
}


/// Reads line from stdin and returns trimed String
/// 
/// Reads line from stdin and returns trimed String.
/// It is similar to `input()` in Python, which returns striped string
///
/// # Examples
/// ```rust
/// let some_useful_string = read_std_line();
/// ```
pub fn read_std_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Couldn`t read std");
    String::from(buffer.trim())
}

/// Returns true if path exists and false if not
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Returns only *.py files of directory
/// 
/// # Examples
/// 
/// ```rust
/// let py_files: Vec<String> = get_py_files("path/to/dir");
/// ```
pub fn get_py_files(path: &str) -> Vec<String> {
    if !exists(path) {
        return vec![];
    }
    let entries = read_dir(path).unwrap();
    let mut files: Vec<String> = vec![];

    for entry in entries {
        let smentry = entry.unwrap().path();
        let en_path = smentry.extension();
        if en_path == None {
            continue;
        }
        if smentry.is_file() && en_path.unwrap() == "py"{
            files.push(String::from(smentry.as_os_str().to_str().unwrap()));
        }
    }

    files
}