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

#[derive(Debug)]
pub enum ParseDirectionError {
    Empty,
    InvalidDirection(char),
    InvalidNumber(ParseIntError),
}

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseDirectionError::Empty => write!(f, "Empty instruction"),
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

fn construct_path(day: u8) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs");
    path.push(format!("day{:02}", day));
    path.push("input.txt");
    path
}

fn buffered_reader(day: u8) -> std::io::Result<impl BufRead> {
    let path = construct_path(day);
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn normalize_dial_position(pos: i32) -> i32 {
    (pos % FULL_ROTATION + FULL_ROTATION) % FULL_ROTATION
}

pub fn count_dial_zero_hits(day: u8, start_value: i32) -> Result<i32, DialError> {
    let reader = buffered_reader(day)?;

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

pub fn count_dial_zero_passes(day: u8, start_value: i32) -> Result<i32, DialError> {
    let reader = buffered_reader(day)?;

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

        if prev_dial_pos > LOWER_BOUNDARY && prev_dial_pos < FULL_ROTATION
            && (curr_dial_pos <= LOWER_BOUNDARY || curr_dial_pos >= FULL_ROTATION) {
            count += 1;
        }

        curr_dial_pos = normalize_dial_position(curr_dial_pos);
    }

    Ok(count)
}
