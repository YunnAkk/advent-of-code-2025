use std::ascii::AsciiExt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    direction: char,
    turns: i32,
}

#[derive(Debug)]
enum ParseDirectionError {
    Empty,
    InvalidDirection(char),
    InvalidNumber(ParseIntError),
}

impl FromStr for Instruction {
    type Err = ParseDirectionError;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let mut chars = instruction.chars();

        // Get the character for the direction
        let first_char = chars.next().ok_or(ParseDirectionError::Empty)?;

        // Validate direction
        let direction = match first_char {
            'L' | 'R' => first_char.to_ascii_uppercase(),
            other => return Err(ParseDirectionError::InvalidDirection(other)),
        };

        let turns = chars
            .as_str()
            .parse::<i32>()
            .map_err(ParseDirectionError::InvalidNumber)?;

        Ok(Instruction { direction, turns })
    }
}

const UPPER_BOUNDARY: i32 = 99;
const LOWER_BOUNDARY: i32 = 0;
const FULL_ROTATION: i32 = UPPER_BOUNDARY - LOWER_BOUNDARY + 1;

pub fn test_day01() {
    let result = read_lines_as_stream(1, 50);
    println!("{result}");
}

fn construct_path(day: u8) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs");
    path.push(format!("day{:02}", day));
    path.push("input.txt");
    path
}

pub fn stream_lines(day: u8) -> std::io::Result<impl BufRead> {
    let path = construct_path(day);
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn normalize_dial_position(pos: i32) -> i32 {
    (pos % FULL_ROTATION + FULL_ROTATION) % FULL_ROTATION
}

pub fn read_lines_as_stream(day: u8, start_value: i32) -> i32 {
    let reader = stream_lines(day).unwrap();

    let mut count = 0;
    let mut curr_dial_pos = start_value;

    // let mut lines: Vec<String> = Vec::new(); Why did we add this?
    for line in reader.lines() {
        // let line = line?; // line is akin to the form of "LXX"
        // println!("{:?}", line);
        let parsed_line = line.unwrap().parse::<Instruction>();
        //let parsed_line = match line {
        //    Ok(line) => line.parse::<Instruction>(),
        //    Err(e) => return Err(e),
        //};

        let instruction = parsed_line.unwrap();

        let delta = if instruction.direction == 'R' { instruction.turns } else { -instruction.turns };
        curr_dial_pos = normalize_dial_position(curr_dial_pos + delta);

        if (curr_dial_pos == 0) {
            count+= 1;
        }
    }

    count
}

