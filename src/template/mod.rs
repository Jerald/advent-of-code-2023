use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod runner;

pub use day::*;

mod day;
mod readme_benchmarks;
mod run_multi;
mod timings;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    let base_dir = {
        let mut cwd = env::current_dir().unwrap();
        if cwd.ends_with("solutions") {
            cwd.pop();
        }

        cwd
    };

    let data_path = base_dir
        .join("data")
        .join(folder)
        .join(format!("{day}.txt"));

    let f = fs::read_to_string(data_path);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
    let base_dir = {
        let mut cwd = env::current_dir().unwrap();
        if cwd.ends_with("solutions") {
            cwd.pop();
        }

        cwd
    };

    let data_path = base_dir
        .join("data")
        .join(folder)
        .join(format!("{day}-{part}.txt"));
    
    let f = fs::read_to_string(data_path);
    f.expect("could not open input file")
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
        const DAY: $crate::template::Day = $crate::day!($day);

        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() -> anyhow::Result<()> {
            use $crate::template::runner::*;
            let input = $crate::template::read_file("inputs", DAY);
            $( let _ = run_part($func, &input, DAY, $part); )*

            Ok(())
        }
    };
}
