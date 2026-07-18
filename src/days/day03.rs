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

    mod separate_string_to_digits {
        use super::*;

        #[test]
        fn empty_string_returns_empty_vec() {
            assert_eq!(separate_string_to_digits(""), vec![]);
        }

        #[test]
        fn no_digits_returns_empty_vec() {
            assert_eq!(separate_string_to_digits("abcXYZ"), vec![]);
        }

        #[test]
        fn mixed_letters_and_digits() {
            assert_eq!(separate_string_to_digits("a1b2c3"), vec![1, 2, 3]);
        }

        #[test]
        fn digits_with_spaces() {
            assert_eq!(separate_string_to_digits("1 2 3"), vec![1, 2, 3]);
        }

        #[test]
        fn single_digit() {
            assert_eq!(separate_string_to_digits("5"), vec![5]);
        }

        #[test]
        fn digits_at_beginning_of_string() {
            assert_eq!(separate_string_to_digits("123abc"), vec![1, 2, 3]);
        }

        #[test]
        fn digits_at_end_of_string() {
            assert_eq!(separate_string_to_digits("abc123"), vec![1, 2, 3]);
        }

        #[test]
        fn zero_digit() {
            assert_eq!(separate_string_to_digits("0"), vec![0]);
        }

        #[test]
        fn multiple_zeros() {
            assert_eq!(separate_string_to_digits("00"), vec![0, 0]);
        }

        #[test]
        fn multiple_digits() {
            let expected = vec![1, 2, 3];
            assert_eq!(separate_string_to_digits("123"), expected);
        }
    }

    mod find_largest_two_digits {
        use super::*;

        #[test]
        fn slash_before_zero_is_excluded() {
            assert_eq!(separate_string_to_digits("/"), vec![]);
        }

        #[test]
        fn colon_after_nine_is_excluded() {
            assert_eq!(separate_string_to_digits(":"), vec![]);
        }

        #[test]
        fn boundary_chars_mixed_with_digits() {
            assert_eq!(separate_string_to_digits("/0:9/"), vec![0, 9]);
        }

        #[test]
        fn unicode_characters_do_not_produce_false_positives() {
            assert_eq!(separate_string_to_digits("é"), vec![]);
        }
    }

    mod calculate_two_digit_joltage {
        use super::*;

        
    }
}
