use crate::scanner::file_model::FileModel;

fn get_file_string_size(size: &u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let mut value = *size as f64;
    let mut unit_index = 0;

    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:.0}{}", value, UNITS[unit_index])
    } else {
        format!("{:.2}{}", value, UNITS[unit_index])
    }
}

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
        let file_icon = match file.is_directory {
            true => "📂",
            false => "📄",
        };

        println!(
            "{}{} - {} {}",
            " ".repeat((depth.unwrap_or(0) * 2) as usize),
            file_icon,
            file.path.to_string_lossy().replace('\\', "/"),
            get_file_string_size(&file.size)
        );
    }
}

pub struct CsvFormatter {
    pub(crate) separator: char,
}
impl Formatter for CsvFormatter {
    fn format(&self, file: &FileModel, _: Option<u32>) {
        let file_type = match file.is_directory {
            true => "directory",
            false => "file",
        };

        println!(
            "{file_type}{sep}{path}{sep}{size}",
            file_type = file_type,
            sep = self.separator,
            path = file.path.to_string_lossy().replace('\\', "/"),
            size = file.size
        );
    }
}
impl CsvFormatter {
    pub const DEFAULT_SEPARATOR: char = ';';

    pub fn print_header(&self) {
        println!("type{sep}path{sep}size", sep = self.separator);
    }
}
