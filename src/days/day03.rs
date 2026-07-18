use crate::days::utils;
use std::io::BufRead;
use std::path::PathBuf;

fn separate_string_to_digits(input: &str) -> Vec<i32> {
    input
        .bytes()
        .filter_map(|b| b.checked_sub(b'0'))
        .filter(|&d| d < 10)
        .map(|d| d as i32)
        .collect()
}

fn find_largest_two_digits(digits_vector: &[i32]) -> (i32, i32) {
    let mut left = digits_vector[0];
    let mut right = digits_vector[1];

    for (&current, &next) in digits_vector
        .iter()
        .zip(digits_vector.iter().skip(1))
        .skip(1)
    {
        if current > left {
            left = current;
            right = next;
        } else if next > right {
            right = next;
        }
    }

    (left, right)
}

pub fn calculate_two_digit_joltage(path: &PathBuf) -> i32 {
    let mut total_sum: i32 = 0;

    let reader = utils::buffered_reader(path).unwrap();

    for line in reader.lines() {
        let digits_vector = separate_string_to_digits(&line.unwrap());
        let (left, right) = find_largest_two_digits(&digits_vector);
        total_sum += left * 10 + right;
    }

    total_sum
}

mod tests {
    use super::*;
}
