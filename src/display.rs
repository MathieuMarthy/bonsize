use crate::cli::Cli;
use crate::scanner::file_model::FileModel;

fn get_file_string_size(file_size: &u64) -> String {
    let suffixes = ["B", "KB", "MB", "GB", "TB"];
    let mut suffix_index: usize = 0;
    let mut size_to_display = *file_size;

    while size_to_display > 1024 && suffix_index < 4 {
        suffix_index += 1;
        size_to_display /= 1024;
    }

    format!("{:.2} {}", size_to_display, suffixes[suffix_index])
}

fn get_file_string(file_model: &FileModel) -> String {
    let file_icon = match file_model.is_directory {
        true => "📂",
        false => "📄",
    };

    format!(
        "{} - {:?} {}",
        file_icon,
        file_model.path,
        get_file_string_size(&file_model.size)
    )
}

pub fn display(file_model: &FileModel, cli_args: &Cli, current_depth: u32) {
    println!(
        "{}{}",
        " ".repeat((current_depth * 2) as usize),
        get_file_string(&file_model)
    );

    for child in &file_model.children {
        display(&child, &cli_args, current_depth + 1);
    }
}
