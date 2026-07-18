//! Secret Entrance — Dial Combinations
//!
//! A safe has a dial numbered 0 through 99. The puzzle input is a sequence of
//! rotation instructions (e.g. `L68`, `R48`) that turn the dial left (toward
//! lower numbers) or right (toward higher numbers) by the given number of clicks.
//! (e.g. left 68 times, right 48 times). When turning left from 0, the dial wraps around
//! to 99 and likewise when turning right from 99 the dial wraps around to 0.
//!
//! # Part One
//! Count how many times the dial ends a rotation pointing at 0.
//!
//! # Part Two
//! Count how many times the dial crosses away from 0 whilst rotating
use crate::days::utils;
use std::fmt;
use std::fmt::Formatter;
use std::io::BufRead;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

/// The lowest number on the dial.
const LOWER_BOUNDARY: i32 = 0;
/// The highest number on the dial.
const UPPER_BOUNDARY: i32 = 99;
/// The number of clicks in a complete rotation.
/// Used for wrapping calculations, turning past 99 goes to 0 and vice versa.
const FULL_ROTATION: i32 = UPPER_BOUNDARY - LOWER_BOUNDARY + 1;

/// Errors that can occur when parsing a rotation instruction.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseDirectionError {
    /// The instruction string was empty.
    Empty,
    /// The first character was not `L` or `R`.
    InvalidDirection(char),
    /// The number of clicks after the direction letter could not be parsed as an integer.
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

/// Top level error type for dial related operations.
///
/// Combines file I/O errors with instruction parsing errors so callers get a
/// single error type.
#[derive(Debug)]
pub enum DialError {
    /// An error occurred while reading the puzzle input file.
    Io(std::io::Error),
    /// An instruction on the given line (1-indexed) failed to parse.
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

/// A single rotation instruction, turn the dial left (`L`) or right (`R`)
/// by a given number of clicks.
///
/// # Format
/// Instructions are strings like `L68` or `R48`: a direction letter
/// followed by a non-negative integer. The letter `L` means rotate toward
/// lower numbers (counter-clockwise on the dial), and `R` means rotate
/// toward higher numbers (clockwise).
#[derive(Debug)]
struct Instruction {
    /// The direction: `L` for left or `R` for right.
    direction: char,
    /// The number of clicks to rotate the dial by.
    turns: i32,
}

impl FromStr for Instruction {
    type Err = ParseDirectionError;

    /// Parses a rotation instruction from a string.
    ///
    /// Expected format: a direction letter (`L` or `R`) followed by a non-negative
    /// integer (e.g. `L68`, `R48`).
    ///
    /// # Errors
    /// Returns [`ParseDirectionError::Empty`], [`ParseDirectionError::InvalidDirection`]
    /// and [`ParseDirectionError::InvalidNumber`]
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

/// Wraps a dial position into the valid range `[0, 99]` using Euclidean modulo.
///
/// The dial is a circle of 100 numbers (0 through 99). Because a rotation can
/// push the position outside this range (e.g. turning left from 2 by 5 clicks
/// produces -3), this function maps any integer position back to `0–99`.
/// Position -1 maps to 99, position 100 maps to 0 and so on.
fn normalize_dial_position(pos: i32) -> i32 {
    (pos % FULL_ROTATION + FULL_ROTATION) % FULL_ROTATION
}

/// Part 1 — counts how many times the dial ends a rotation pointing at 0.
///
/// Reads rotation instructions from the file at `path` (one instruction per line,
/// e.g. `L68`). Starting from `start_value`, 50 in the puzzle, each instruction
/// is applied in order. After each complete instruction, if the dial's final
/// position lands exactly on 0, the count is incremented.
///
/// # Example
/// With start_value = 50 and the instructions `L68`, `L30`, `R48` and `L5`:
///
/// | Pos   | Instruction | End | Hit? |
/// |-------|-------------|-----|------|
/// | 50    | L68         | 82  |      |
/// | 82    | L30         | 52  |      |
/// | 52    | R48         | 0   | ✓    |
/// | 0     | L5          | 95  |      |
///
/// The function returns 1.
///
/// # Errors
/// Returns [`DialError`] if the file cannot be read or any line contains an
/// invalid instruction.
pub fn count_dial_zero_hits(path: &PathBuf, start_value: i32) -> Result<i32, DialError> {
    let reader = utils::buffered_reader(path)?;

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

/// Part 2 — counts how many times the dial crosses away from 0 whilst rotating.
///
/// If a rotation causes the dial to rotate past 0, every such crossing is counted.
/// For example, starting at 50 and turning right by 1000 clicks will complete 10 full laps
/// of the dial, passing 0 ten times before returning to 50. This is the key difference from Part 1.
/// We don't count how many times the dial lands at 0, but rather how many times it goes away from 0
///
/// # How it works
/// 1. Each full rotation (every 100 clicks of a turn) is guaranteed to cross 0
///    exactly once, so `turns / 100` is added to the count immediately.
/// 2. For the remaining partial rotation (`turns % 100`), the function checks
///    whether the dial moved past the wraparound boundary during the move from
///    `prev_dial_pos` to `curr_dial_pos`.
/// 3. The position is then normalized into `[0, 99]`.
///
/// # Errors
/// Returns [`DialError`] if the file cannot be read or any line contains an
/// invalid instruction.
pub fn count_dial_zero_passes(path: &PathBuf, start_value: i32) -> Result<i32, DialError> {
    let reader = utils::buffered_reader(path)?;

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
    use super::*;
    use crate::define_day_in_path;

    define_day_in_path!("01");

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
