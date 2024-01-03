use solutions::prelude::*;

use std::str::FromStr;

use anyhow::{bail, Context};

advent_of_code::solution!(2);

type Num = u32;

#[derive(Debug)]
struct GameLine {
    id: Num,
    revealed_cubes: Vec<CubeSet>,
}

impl FromStr for GameLine {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_tag, reveals) = s
            .split_once(':')
            .with_context(|| "Unable to split line by ':'")?;

        let id = game_tag
            .strip_prefix("Game")
            .with_context(|| "Unable to remove 'Game' prefix!")?
            .trim()
            .parse::<Num>()?;

        let cube_sets = reveals
            .split(';')
            .map(str::parse::<CubeSet>)
            .collect::<anyhow::Result<_>>()?;

        Ok(GameLine {
            id,
            revealed_cubes: cube_sets,
        })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CubeCount {
    Red(Num),
    Green(Num),
    Blue(Num),
}

impl CubeCount {
    fn parse_cube(count: Num, cube: &str) -> anyhow::Result<Self> {
        let builder = match cube.to_lowercase().trim() {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => bail!("Input '{}' is not a cube colour!", cube),
        };

        Ok(builder(count))
    }
}

impl FromStr for CubeCount {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.to_lowercase().trim().to_owned();

        let (count, cube) = input
            .split_once(' ')
            .with_context(|| format!("Unable to split reveal string by space: '{}'", s))?;

        let count = count
            .parse::<Num>()
            .with_context(|| format!("Couldn't parse count for reveal: '{}'", count))?;

        CubeCount::parse_cube(count, cube).context("Couldn't parse cube for reveal!")
    }
}

pub fn part_one(input: &str) -> anyhow::Result<Num> {
    let games = input.lines().map(str::parse::<GameLine>);

    const MAX_SET: CubeSet = CubeSet {
        red: Some(12),
        green: Some(13),
        blue: Some(14),
    };

    games
        .map(|game| {
            let GameLine {
                id,
                revealed_cubes: cube_reveals,
            } = game?;
            anyhow::Ok((id, cube_reveals.find_set_of_max()))
        })
        .filter(|pair| {
            let Ok((_, game_max)) = pair else { return true };
            game_max.is_possible_with_max(MAX_SET)
        })
        .map(|pair| anyhow::Ok(pair?.0))
        .sum()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
struct CubeSet {
    red: Option<Num>,
    green: Option<Num>,
    blue: Option<Num>,
}

impl CubeSet {
    const fn empty() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
        }
    }

    fn with_cube(self, cube: CubeCount) -> Self {
        use CubeCount::*;
        match cube {
            Red(count) => Self {
                red: Some(count),
                ..self
            },
            Green(count) => Self {
                green: Some(count),
                ..self
            },
            Blue(count) => Self {
                blue: Some(count),
                ..self
            },
        }
    }

    fn power(&self) -> Num {
        let CubeSet { red, green, blue } = self;

        assert!(
            !(red.is_none() && green.is_none() && blue.is_none()),
            "Shouldn't use power on an empty cube set!"
        );

        let red = red.unwrap_or(1);
        let green = green.unwrap_or(1);
        let blue = blue.unwrap_or(1);

        red * green * blue
    }

    fn element_wise_max(value1: Self, value2: Self) -> Self {
        let CubeSet {
            red: r1,
            green: g1,
            blue: b1,
        } = value1;
        let CubeSet {
            red: r2,
            green: g2,
            blue: b2,
        } = value2;

        CubeSet {
            red: r1.max(r2),
            green: g1.max(g2),
            blue: b1.max(b2),
        }
    }

    fn is_possible_with_max(self, max: Self) -> bool {
        let colours = [
            self.red.zip(max.red),
            self.green.zip(max.green),
            self.blue.zip(max.blue),
        ];

        colours
            .into_iter()
            .filter_map(|colour| colour.map(|(this, max)| this <= max))
            .all(std::convert::identity)
    }
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cubes = input
            .split(',')
            .map(str::trim)
            .map(str::to_lowercase)
            .map(|s| s.parse::<CubeCount>());

        #[cfg(debug_assertions)]
        {
            let cube_vec: Vec<_> = cubes.clone().collect::<anyhow::Result<_>>()?;
            assert!(
                cube_vec.len() <= 3,
                "Cube set is expected to not contain duplicated. Unless it should?"
            );
        }

        cubes.try_fold(CubeSet::default(), |cube_set, cube| {
            Ok(cube_set.with_cube(cube?))
        })
    }
}

trait CubeSetMax {
    fn find_set_of_max(&self) -> CubeSet;
}

impl CubeSetMax for Vec<CubeSet> {
    fn find_set_of_max(&self) -> CubeSet {
        self.iter()
            .copied()
            .fold(CubeSet::empty(), CubeSet::element_wise_max)
    }
}

pub fn part_two(input: &str) -> anyhow::Result<Num> {
    let games = input.lines().map(str::parse::<GameLine>);

    games
        .debug_inspect(|game| println!("Game lines: {game:?}"))
        .map(|game| anyhow::Ok(game?.revealed_cubes.find_set_of_max()))
        .debug_inspect(|max_set| println!("Max set: {max_set:?}"))
        .map(|max_set| anyhow::Ok(max_set?.power()))
        .debug_inspect(|power| println!("Max set power: {power:?}"))
        // Sum the powers
        .try_fold(0, |acc, new| Ok(acc + new?))
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
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &template::read_data_file(DataFolder::Examples, DataFile::DayPart(DAY, 2)).unwrap(),
        )
        .unwrap();
        assert_eq!(result, 2286);
    }
}
