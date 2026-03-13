use bonsize_core::scanner::file_model::FileModel;

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
    fn format(&self, file: &FileModel);
}

pub struct TreeFormatter {
    pub use_indentation: bool
}
impl Formatter for TreeFormatter {
    fn format(&self, file: &FileModel) {
        let file_icon = match file.is_directory {
            true => "📂",
            false => "📄",
        };

        println!(
            "{}{} - {} {}",
            if self.use_indentation {" ".repeat(file.depth * 2)} else { String::from("") },
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
    fn format(&self, file: &FileModel) {
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
