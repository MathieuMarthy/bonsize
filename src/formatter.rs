use crate::display::{get_file_csv_string, get_file_string};
use crate::scanner::file_model::FileModel;

pub trait Formatter {
    /// Formats and outputs information about a single file.
    ///
    /// * `file` is the file entry to be formatted.
    /// * `depth` is the logical depth of `file` in a tree structure:
    ///   when rendering a hierarchical tree, pass `Some(level)` to
    ///   control indentation (e.g. the root at depth 0, its children
    ///   at depth 1, and so on). For flat, non-hierarchical output
    ///   (such as CSV lists), pass `None` and implementations are
    ///   expected to ignore the depth.
    fn format(&self, file: &FileModel, depth: Option<u32>);
}

pub struct TreeFormatter;
impl Formatter for TreeFormatter {
    fn format(&self, file: &FileModel, depth: Option<u32>) {
        println!(
            "{}{}",
            " ".repeat((depth.unwrap_or(0) * 2) as usize),
            get_file_string(&file)
        );
    }
}

pub struct CsvFormatter;
impl Formatter for CsvFormatter {
    fn format(&self, file: &FileModel, _: Option<u32>) {
        println!("{}", get_file_csv_string(&file))
    }
}
impl CsvFormatter {
    pub fn print_header(&self) {
        println!("type;path;size");
    }
}
