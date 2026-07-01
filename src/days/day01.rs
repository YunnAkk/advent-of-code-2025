use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

const UPPER_BOUNDARY: i32 = 99;
const LOWER_BOUNDARY: i32 = 0;
const FULL_ROTATION: i32 = UPPER_BOUNDARY - LOWER_BOUNDARY + 1;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseDirectionError {
    Empty,
    InvalidDirection(char),
    InvalidNumber(ParseIntError),
}

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseDirectionError::Empty => write!(f, "Missing instruction"),
            ParseDirectionError::InvalidDirection(char) => write!(f, "Invalid direction: {char}"),
            ParseDirectionError::InvalidNumber(num) => write!(f, "invalid number: {num}"),
        }
    }
}

#[derive(Debug)]
pub enum DialError {
    Io(std::io::Error),
    Parse(usize, ParseDirectionError),
}

impl fmt::Display for DialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DialError::Io(e) => write!(f, "I/O error: {e}"),
            DialError::Parse(line, e) => write!(f, "Error on line {line}: {e}"),
        }
    }
}

impl From<std::io::Error> for DialError {
    fn from(value: std::io::Error) -> Self {
        DialError::Io(value)
    }
}

#[derive(Debug)]
struct Instruction {
    direction: char,
    turns: i32,
}

impl FromStr for Instruction {
    type Err = ParseDirectionError;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let direction = match instruction.as_bytes().first() {
            Some(&b @ (b'L' | b'R')) => b as char,
            Some(&other) => return Err(ParseDirectionError::InvalidDirection(other as char)),
            None => return Err(ParseDirectionError::Empty),
        };

        let turns = instruction[1..]
            .parse::<i32>()
            .map_err(ParseDirectionError::InvalidNumber)?;

        Ok(Instruction { direction, turns })
    }
}

fn buffered_reader(path: &PathBuf) -> std::io::Result<impl BufRead> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn normalize_dial_position(pos: i32) -> i32 {
    (pos % FULL_ROTATION + FULL_ROTATION) % FULL_ROTATION
}

pub fn count_dial_zero_hits(path: &PathBuf, start_value: i32) -> Result<i32, DialError> {
    let reader = buffered_reader(path)?;

    let mut count = 0;
    let mut curr_dial_pos = start_value;

    for (i, line) in reader.lines().enumerate() {
        let instruction = line?
            .parse::<Instruction>()
            .map_err(|e| DialError::Parse(i + 1, e))?;

        let delta = match instruction.direction {
            'R' => instruction.turns,
            _ => -instruction.turns,
        };

        curr_dial_pos = normalize_dial_position(curr_dial_pos + delta);

        if curr_dial_pos == 0 {
            count += 1;
        }
    }

    Ok(count)
}

pub fn count_dial_zero_passes(path: &PathBuf, start_value: i32) -> Result<i32, DialError> {
    let reader = buffered_reader(path)?;

    let mut count = 0;
    let mut curr_dial_pos = start_value;

    for (i, line) in reader.lines().enumerate() {
        let instruction = line?
            .parse::<Instruction>()
            .map_err(|e| DialError::Parse(i + 1, e))?;

        let quotient = instruction.turns / FULL_ROTATION;
        let remainder = instruction.turns % FULL_ROTATION;

        count += quotient;

        let prev_dial_pos = curr_dial_pos;
        let delta = match instruction.direction {
            'R' => remainder,
            _ => -remainder,
        };
        curr_dial_pos += delta;

        if prev_dial_pos > LOWER_BOUNDARY
            && prev_dial_pos < FULL_ROTATION
            && (curr_dial_pos <= LOWER_BOUNDARY || curr_dial_pos >= FULL_ROTATION)
        {
            count += 1;
        }

        curr_dial_pos = normalize_dial_position(curr_dial_pos);
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::super::utils;
    use super::*;

    fn path_for(name: &str) -> PathBuf {
        utils::get_path_from_root(&format!("test_inputs/day01/{name}"))
    }

    mod count_dial_zero_hits {
        use super::*;

        #[test]
        fn from_0_to_99() {
            assert_eq!(
                count_dial_zero_hits(&path_for("0-to-99.txt"), 0).unwrap(),
                1,
            );
        }

        #[test]
        fn from_1_to_99() {
            assert_eq!(
                count_dial_zero_hits(&path_for("1-to-99.txt"), 1).unwrap(),
                1,
            );
        }

        #[test]
        fn from_98_to_0() {
            assert_eq!(
                count_dial_zero_hits(&path_for("98-to-0.txt"), 98).unwrap(),
                2,
            );
        }

        #[test]
        fn from_99_to_0() {
            assert_eq!(
                count_dial_zero_hits(&path_for("99-to-0.txt"), 99).unwrap(),
                2,
            );
        }

        #[test]
        fn aoc_example() {
            assert_eq!(
                count_dial_zero_hits(&path_for("aoc_example.txt"), 50).unwrap(),
                3,
            );
        }

        #[test]
        fn full_rotation_left() {
            assert_eq!(
                count_dial_zero_hits(&path_for("full-rotation-left.txt"), 0).unwrap(),
                1,
            );
        }

        #[test]
        fn full_rotation_right() {
            assert_eq!(
                count_dial_zero_hits(&path_for("full-rotation-right.txt"), 0).unwrap(),
                1,
            );
        }

        #[test]
        fn simple_back_forth() {
            assert_eq!(
                count_dial_zero_hits(&path_for("simple-back-forth.txt"), 10).unwrap(),
                160,
            );
        }
    }

    mod count_dial_zero_passes {
        use super::*;

        #[test]
        fn from_0_to_99() {
            assert_eq!(
                count_dial_zero_passes(&path_for("0-to-99.txt"), 0).unwrap(),
                1,
            );
        }

        #[test]
        fn from_1_to_99() {
            assert_eq!(
                count_dial_zero_passes(&path_for("1-to-99.txt"), 1).unwrap(),
                2,
            );
        }

        #[test]
        fn from_98_to_0() {
            assert_eq!(
                count_dial_zero_passes(&path_for("98-to-0.txt"), 98).unwrap(),
                2,
            );
        }

        #[test]
        fn from_99_to_0() {
            assert_eq!(
                count_dial_zero_passes(&path_for("99-to-0.txt"), 99).unwrap(),
                2,
            );
        }

        #[test]
        fn aoc_example() {
            assert_eq!(
                count_dial_zero_passes(&path_for("aoc_example.txt"), 50).unwrap(),
                6,
            );
        }

        #[test]
        fn full_rotation_left() {
            assert_eq!(
                count_dial_zero_passes(&path_for("full-rotation-left.txt"), 2).unwrap(),
                2,
            );
        }

        #[test]
        fn full_rotation_right() {
            assert_eq!(
                count_dial_zero_passes(&path_for("full-rotation-right.txt"), 2).unwrap(),
                1,
            );
        }

        #[test]
        fn simple_back_forth() {
            assert_eq!(
                count_dial_zero_passes(&path_for("simple-back-forth.txt"), 10).unwrap(),
                160,
            );
        }
    }

    mod parse_instruction {
        use super::*;

        #[test]
        fn rejects_empty_string() {
            let result = Instruction::from_str("");

            let e = result.unwrap_err();

            assert_eq!(e, ParseDirectionError::Empty);
        }

        #[test]
        fn rejects_unknown_direction() {
            let result = Instruction::from_str("X10");

            let e = result.unwrap_err();

            assert_eq!(e, ParseDirectionError::InvalidDirection('X'));
        }

        #[test]
        fn rejects_non_numeric_turns() {
            let result = Instruction::from_str("Rnumber");

            let e = result.unwrap_err();

            let expected_error = "number".parse::<i32>().unwrap_err();

            assert_eq!(e, ParseDirectionError::InvalidNumber(expected_error));
        }
    }
}
