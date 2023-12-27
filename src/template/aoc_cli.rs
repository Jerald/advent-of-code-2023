/// Wrapper module around the "aoc-cli" command-line.
use std::{
    ffi::OsString,
    fmt::Display,
    process::{Command, Output, Stdio},
};

use crate::template::{DataFolder, Day};

#[derive(Debug)]
pub enum AocCommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExitStatus(Output),
}

impl Display for AocCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocCommandError::CommandNotFound => write!(f, "aoc-cli is not present in environment."),
            AocCommandError::CommandNotCallable => write!(f, "aoc-cli could not be called."),
            AocCommandError::BadExitStatus(_) => {
                write!(f, "aoc-cli exited with a non-zero status.")
            }
        }
    }
}

pub fn check() -> Result<(), AocCommandError> {
    Command::new("aoc")
        .arg("-V")
        .output()
        .map_err(|_| AocCommandError::CommandNotFound)?;
    Ok(())
}

pub fn read(day: Day) -> Result<Output, AocCommandError> {
    let args = build_args(
        "read",
        [
            "--description-only".into(),
            "--puzzle-file".into(),
            DataFolder::Puzzles.path_with(day).into(),
        ]
        .into_iter(),
        day,
    );

    call_aoc_cli(args)
}

pub fn download(day: Day) -> Result<Output, AocCommandError> {
    let input_path = DataFolder::Inputs.path_with(day);
    let puzzle_path = DataFolder::Puzzles.path_with(day);

    let args = build_args(
        "download",
        [
            "--overwrite".into(),
            "--input-file".into(),
            input_path.clone().into(),
            "--puzzle-file".into(),
            puzzle_path.clone().into(),
        ].into_iter(),
        day,
    );

    let output = call_aoc_cli(args)?;
    println!("---");
    println!(
        "🎄 Successfully wrote input to \"{}\".",
        input_path.display()
    );
    println!(
        "🎄 Successfully wrote puzzle to \"{}\".",
        puzzle_path.display()
    );
    Ok(output)
}

pub fn submit(day: Day, part: u8, result: &str) -> Result<Output, AocCommandError> {
    // workaround: the argument order is inverted for submit.
    let args = build_args("submit", std::iter::empty(), day)
        .chain([part.to_string().into(), result.to_string().into()]);

    call_aoc_cli(args)
}

fn get_year() -> Option<u16> {
    match std::env::var("AOC_YEAR") {
        Ok(x) => x.parse().ok().or(None),
        Err(_) => None,
    }
}

fn build_args(
    command: &str,
    args: impl Iterator<Item = OsString>,
    day: Day,
) -> impl Iterator<Item = OsString> {
    let mut extra_args = Vec::new();

    if let Some(year) = get_year() {
        extra_args.push("--year".into());
        extra_args.push(year.to_string().into());
    }

    extra_args.append(&mut vec![
        "--day".into(),
        day.to_string().into(),
        command.into(),
    ]);

    args.chain(extra_args)
}

fn call_aoc_cli(args: impl Iterator<Item = OsString>) -> Result<Output, AocCommandError> {
    // println!("Calling >aoc with: {}", args.join(" "));
    let output = Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| AocCommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(AocCommandError::BadExitStatus(output))
    }
}
