use crate::cli::{Cli, Sort};
use crate::display::formatter::{CsvFormatter, TreeFormatter};
use crate::display::{display_as_sorted_list, display_as_tree};
use crate::scanner::file_model::FileModel;

pub enum OutputFormat {
    Tree,
    SortedList,
    CsvTree,
    CsvList,
}

impl OutputFormat {
    pub fn from_args(sort: &Option<Sort>, display_as_csv: Option<char>) -> Self {
        match (sort, display_as_csv) {
            (Some(_), Some(_)) => OutputFormat::CsvList,
            (None, Some(_)) => OutputFormat::CsvTree,
            (Some(_), _) => OutputFormat::SortedList,
            (None, _) => OutputFormat::Tree,
        }
    }

    pub fn display(&self, file: &FileModel, args: &Cli) {
        match self {
            OutputFormat::CsvList => {
                let formatter = CsvFormatter {
                    separator: args.csv.unwrap_or(CsvFormatter::DEFAULT_SEPARATOR),
                };
                formatter.print_header();
                display_as_sorted_list(file, args, &formatter);
            }
            OutputFormat::CsvTree => {
                let formatter = CsvFormatter {
                    separator: args.csv.unwrap_or(CsvFormatter::DEFAULT_SEPARATOR),
                };
                formatter.print_header();
                display_as_tree(file, args, 0, &formatter);
            }
            OutputFormat::SortedList => {
                let formatter = TreeFormatter;
                display_as_sorted_list(file, args, &formatter)
            }
            OutputFormat::Tree => {
                let formatter = TreeFormatter;
                display_as_tree(file, args, 0, &formatter)
            }
        };
    }
}
