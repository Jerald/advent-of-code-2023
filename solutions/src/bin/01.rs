use std::fmt::Display;
use std::ops::Range;

use anyhow::Context;

advent_of_code::solution!(1);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    pub const fn name(self) -> &'static str {
        use Digit::*;
        match self {
            One => "one",
            Two => "two",
            Three => "three",
            Four => "four",
            Five => "five",
            Six => "six",
            Seven => "seven",
            Eight => "eight",
            Nine => "nine",
        }
    }

    pub const fn number(self) -> &'static str {
        use Digit::*;
        match self {
            One => "1",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
        }
    }

    /// Returns a digit if one is contained in the string.
    ///
    /// Used correctly, the input string should only contain one digit. Otherwise the output may be unexpected.
    pub fn parse_part_2(input: &str) -> Option<Self> {
        use Digit::*;
        [One, Two, Three, Four, Five, Six, Seven, Eight, Nine]
            .into_iter()
            .find(|&digit| input.contains(digit.name()) || input.contains(digit.number()))
    }

    /// Returns a digit if one is contained in the string.
    ///
    /// Used correctly, the input string should only contain one digit. Otherwise the output may be unexpected.
    ///
    /// Designed for part 1, only checks for digit based on the digit itself, not the spelled word.
    pub fn parse_part_1(input: &str) -> Option<Self> {
        use Digit::*;
        [One, Two, Three, Four, Five, Six, Seven, Eight, Nine]
            .into_iter()
            .find(|&digit| input.contains(digit.number()))
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number())
    }
}

struct FirstLastDigitIter<'a> {
    line: &'a str,
    front_iter: Range<usize>,
    back_iter: std::iter::Rev<Range<usize>>,
}

impl<'a> FirstLastDigitIter<'a> {
    pub fn new(line: &'a str) -> Self {
        FirstLastDigitIter {
            front_iter: 0..line.len(),
            back_iter: (0..line.len()).rev(),
            line,
        }
    }
}

impl<'a> Iterator for FirstLastDigitIter<'a> {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            line, front_iter, ..
        } = self;

        for idx in front_iter {
            if let out @ Some(_) = Digit::parse_part_2(&line[..=idx]) {
                return out;
            }
        }

        None
    }
}

impl<'a> DoubleEndedIterator for FirstLastDigitIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Self {
            line, back_iter, ..
        } = self;

        for idx in back_iter {
            if let out @ Some(_) = Digit::parse_part_2(&line[idx..]) {
                return out;
            }
        }

        None
    }
}

fn solution_base<'a>(lines: impl Iterator<Item = &'a str>) -> anyhow::Result<u32> {
    let mut calibration_values = lines.map(|line| {
        let mut iter = FirstLastDigitIter::new(line);

        let first = iter
            .next()
            .context(format!("no first digit found! Line: {}", line))?;
        let last = iter.next_back().unwrap_or(first);

        let result = format!("{first}{last}")
            .parse::<u32>()
            .context("couldn't parse combined digits");

        if let Ok(num) = result {
            println!(
                "Line '{}' has digits {first} and {last} with value {num}",
                line
            )
        }

        result
    });

    calibration_values.try_fold(0, |acc, result| result.map(|val| acc + val))
}

pub fn part_one(input: &str) -> anyhow::Result<u32> {
    solution_base(input.lines())
}

pub fn part_two(input: &str) -> anyhow::Result<u32> {
    solution_base(input.lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::{self, DataFile, DataFolder};

    #[test]
    fn test_part_one() {
        let result = part_one(
            &template::read_data_file(DataFolder::Examples, DataFile::DayPart(DAY, 1)).unwrap(),
        )
        .unwrap();
        assert_eq!(result, 142)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &template::read_data_file(DataFolder::Examples, DataFile::DayPart(DAY, 2)).unwrap(),
        )
        .unwrap();
        assert_eq!(result, 281);
    }
}
