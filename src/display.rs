use crate::cli::{Cli, Sort};
use crate::scanner::file_model::FileModel;
use std::cmp::Reverse;

const BYTES_PER_UNIT: u64 = 1024;
const SUFFIXES: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

fn get_file_string_size(file_size: &u64) -> String {
    let mut suffix_index: usize = 0;
    let mut size_to_display = *file_size;

    while size_to_display > BYTES_PER_UNIT && suffix_index < SUFFIXES.len() - 1 {
        suffix_index += 1;
        size_to_display /= BYTES_PER_UNIT;
    }

    format!("{:.2} {}", size_to_display, SUFFIXES[suffix_index])
}

fn get_file_string(file_model: &FileModel) -> String {
    let file_icon = match file_model.is_directory {
        true => "📂",
        false => "📄",
    };

    format!(
        "{} - {} {}",
        file_icon,
        file_model.path.to_string_lossy().replace('\\', "/"),
        get_file_string_size(&file_model.size)
    )
}

pub fn display(file_model: &FileModel, cli_args: &Cli, current_depth: u32) {
    if cli_args.max_depth.is_some_and(|max| current_depth >= max) {
        return;
    }

    if (!file_model.is_directory && !cli_args.show_only_dir)
        || (file_model.is_directory && !cli_args.show_only_files)
    {
        println!(
            "{}{}",
            " ".repeat((current_depth * 2) as usize),
            get_file_string(&file_model)
        );
    }

    for child in &file_model.children {
        display(&child, &cli_args, current_depth + 1);
    }
}

pub fn display_as_sorted_list(file_model: &FileModel, cli_args: &Cli) {
    let mut all_files: Vec<&FileModel> = Vec::new();
    file_model.get_flattened_files(&mut all_files);

    match cli_args.sort {
        Some(Sort::Asc) => all_files.sort_by_key(|file| file.size),
        _ => all_files.sort_by_key(|file| Reverse(file.size)),
    }

    for file in all_files {
        if (!file.is_directory && !cli_args.show_only_dir)
            || (file.is_directory && !cli_args.show_only_files)
        {
            println!("{}", get_file_string(&file))
        }
    }
}
