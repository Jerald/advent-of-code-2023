use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod runner;

use anyhow::Context;
pub use day::*;

mod day;
mod readme_benchmarks;
mod run_multi;
mod timings;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy)]
pub enum DataFolder {
    Examples,
    Inputs,
    Puzzles,
}

impl DataFolder {
    /// Returns the **relative** path of the data folder in `self`. It's relative to the package root.
    fn sub_directory(self) -> &'static Path {
        Path::new(match self {
            Self::Examples => "./data/examples",
            Self::Inputs => "./data/inputs",
            Self::Puzzles => "./data/puzzles",
        })
    }

    /// Provides the extension that's expected to be contained in a given data folder
    fn expected_extension(self) -> &'static str {
        match self {
            Self::Examples => "txt",
            Self::Inputs => "txt",
            Self::Puzzles => "md",
        }
    }

    /// Constructs the absolute path to the data folder in `self`.
    fn data_path(self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(self.sub_directory())
    }

    /// Constructs the **relative** path to the specified data file in this data folder.
    ///
    /// As with [Self::sub_directory], this is relative to the package root.
    fn path_with(self, file: impl Into<DataFile>) -> PathBuf {
        let file = file.into();

        self.sub_directory()
            .join(file.as_path(self.expected_extension()))
    }

    /// Constructs the **absolute** path to the specified data file in this data folder.
    fn data_path_with(self, file: impl Into<DataFile>) -> PathBuf {
        let file = file.into();

        self.data_path()
            .join(file.as_path(self.expected_extension()))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DataFile {
    Day(Day),
    DayPart(Day, u8),
}

impl DataFile {
    fn as_path(&self, extension: impl AsRef<OsStr>) -> PathBuf {
        let file_name = match self {
            Self::Day(d) => PathBuf::from(d.to_string()),
            Self::DayPart(d, p) => PathBuf::from(format!("{d}-{p}")),
        };

        file_name.with_extension(extension)
    }
}

impl From<Day> for DataFile {
    fn from(input: Day) -> Self {
        Self::Day(input)
    }
}

/// Helper to read a given data file from a given data folder
pub fn read_data_file(folder: DataFolder, file: DataFile) -> anyhow::Result<String> {
    fs::read_to_string(folder.data_path_with(file))
        .with_context(|| format!("couldn't get data from path '{:?}'", folder.path_with(file)))
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
///
/// The optional, second parameter (1 or 2) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        $crate::solution!(@impl $day, [part_one, 1] [part_two, 2]);
    };
    ($day:expr, 1) => {
        $crate::solution!(@impl $day, [part_one, 1]);
    };
    ($day:expr, 2) => {
        $crate::solution!(@impl $day, [part_two, 2]);
    };

    (@impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        /// The current day.
        const DAY: $crate::template::Day = match $crate::template::Day::new($day) {
            Some(day) => day,
            None => panic!(concat!("Not a valid day: ", $day))
        };

        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() -> anyhow::Result<()> {
            use $crate::template::runner::*;
            let input = $crate::template::read_data_file($crate::template::DataFolder::Inputs, $crate::template::DataFile::Day(DAY))?;
            $( let _ = run_part($func, &input, DAY, $part); )*

            Ok(())
        }
    };
}
