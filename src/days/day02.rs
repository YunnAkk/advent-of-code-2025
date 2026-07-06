use crate::days::utils::buffered_reader;
use std::io::BufRead;
use std::path::PathBuf;

fn parse_range(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    if let Some(hyphen_idx) = bytes.iter().position(|&b| b == b'-') {
        let start_bytes = &bytes[..hyphen_idx];
        let end_bytes = &bytes[(hyphen_idx + 1)..];

        Some((start_bytes, end_bytes))
    } else {
        None
    }
}

fn parse_int(bytes: &[u8]) -> Option<i64> {
    let mut val: i64 = 0;
    let mut has_digits = false;

    for &b in bytes {
        if b.is_ascii_digit() {
            val = val * 10 + (b - b'0') as i64;
            has_digits = true;
        } else if b.is_ascii_whitespace() {
            continue;
        } else {
            return None;
        }
    }
    if has_digits { Some(val) } else { None }
}

fn sum_invalid_in_range(start: i64, end: i64) -> i64 {
    let mut total_sum = 0;
    let mut current_num = start;

    while current_num <= end {
        let num_digits = match current_num.checked_ilog10() {
            Some(log) => log + 1,
            None if current_num == 0 => 1,
            _ => panic!("Negative number error"),
        };

        if num_digits >= 19 {
            panic!("The number in the given range does not fit in an i64");
        }

        if (num_digits % 2) == 0 {
            let half_len = num_digits / 2;
            let half_base = 10_i64.pow(half_len);
            let left_half = current_num / half_base;
            let right_half = current_num % half_base;

            if left_half > right_half {
                current_num += left_half - right_half;
            } else if left_half == right_half {
                total_sum += current_num;
                current_num += half_base + 1;
            } else if left_half < right_half {
                let next_left_half = left_half + 1;
                let right_deficit = half_base - right_half;
                current_num += right_deficit + next_left_half;
            }
        } else {
            let next_pow10 = 10_i64.pow(num_digits);
            current_num = next_pow10;
        }
    }

    total_sum
}

pub fn sum_invalid_ids_in_ranges(path: &PathBuf) -> i64 {
    let mut reader = buffered_reader(path).unwrap();
    let mut buffer = Vec::new();
    let mut invalid_ids_sum: i64 = 0;

    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b',', &mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        let mut slice = &buffer[..];

        if slice.ends_with(&[b',']) {
            slice = &slice[..(slice.len() - 1)];
        }

        if let Some((start_bytes, end_bytes)) = parse_range(slice) {
            if let (Some(start), Some(end)) = (parse_int(start_bytes), parse_int(end_bytes)) {
                invalid_ids_sum += sum_invalid_in_range(start, end);
            }
        }
    }

    invalid_ids_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::utils::get_path_from_root;

    mod invalid_ids_single_sequence {
        use super::*;

        #[test]
        fn aoc_example() {
            let path = get_path_from_root("test_inputs/day02/aoc_example.txt");
            assert_eq!(sum_invalid_ids_in_ranges(&path), 1227775554);
        }

        #[test]
        fn left_equal_right() {
            let path = get_path_from_root("test_inputs/day02/left_equal_right.txt");
            assert_eq!(sum_invalid_ids_in_ranges(&path), 48985);
        }

        #[test]
        fn left_greater() {
            let path = get_path_from_root("test_inputs/day02/left_greater_right.txt");
            assert_eq!(sum_invalid_ids_in_ranges(&path), 4444);
        }

        #[test]
        fn left_lesser() {
            let path = get_path_from_root("test_inputs/day02/left_less_right.txt");
            assert_eq!(sum_invalid_ids_in_ranges(&path), 2222);
        }

        #[test]
        fn no_invalid() {
            let path = get_path_from_root("test_inputs/day02/no_invalid.txt");
            assert_eq!(sum_invalid_ids_in_ranges(&path), 0);
        }

        #[test]
        fn single_invalid() {
            let path = get_path_from_root("test_inputs/day02/single_invalid.txt");
            assert_eq!(sum_invalid_ids_in_ranges(&path), 11);
        }

        #[test]
        fn many_sequences() {
            let path = get_path_from_root("test_inputs/day02/many_sequences.txt");
            assert_eq!(sum_invalid_ids_in_ranges(&path), 24416684186);
        }
    }

    mod invalid_ids_repeated_sequence {}
}
