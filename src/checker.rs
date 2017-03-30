// extern crate regex;
use regex::Regex;
//extern crate io_tools;
pub use io_tools::read_str;


/// Reads file and return entries of io operations in it
/// 
/// # Examples
///
/// ```rust
/// let ios: Vec<String> = check_ios("path/to/file");
/// ```
pub fn check_ios(path: &str) -> Vec<String> {
    let file_content = read_str(path);
    let file_lines: Vec<&str> = file_content.split("\n").collect();

    let fwrites = Regex::new(r"\w+\.write[(].+[)]").unwrap();
    let freads = Regex::new(r"\w+\.read[(].*[)]").unwrap();
    let prints = Regex::new(r"print[(].+[)]").unwrap();
    
    let mut findings: Vec<String> = vec![];

    for linen in 0..file_lines.len() {
        let line = file_lines[linen].trim();
        if fwrites.is_match(line) && !line.contains("# safe") {
            findings.push(format!("`{}`: fwrite at {}:  `{}`", path, linen, line));
        } else if freads.is_match(line) && !line.contains("# safe") {
            findings.push(format!("`{}`: fread at {}:   `{}`", path, linen, line));
        } else if prints.is_match(line) && !line.contains("# safe") {
            findings.push(format!("`{}`: print at {}:   `{}`", path, linen, line));
        }
    }

    findings
}
