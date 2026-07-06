use crate::days::utils::buffered_reader;
use std::io::BufRead;
use std::path::PathBuf;

fn process_chunk(ranges_str: &str, invalid_ids_sum: &mut i64) {
    // TODO: remove unwrap
    for range in ranges_str.split(',') {
        if range.is_empty() {
            continue;
        }

        let mut bounds = range.split('-');
        let start_str = bounds.next().unwrap();
        let end_str = bounds.next().unwrap();
        let mut current_num = start_str.parse::<i64>().unwrap();
        let end_num = end_str.parse::<i64>().unwrap();

        while current_num <= end_num {
            let num_digits = current_num.ilog10() + 1;

            if (num_digits % 2) == 0 {
                let half_len = num_digits / 2;
                let half_base = 10_i64.pow(half_len);
                let left_half = current_num / half_base;
                let right_half = current_num % half_base;

                if left_half > right_half {
                    current_num += left_half - right_half;
                } else if left_half == right_half {
                    *invalid_ids_sum += current_num;
                    current_num += half_base;
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
    }
}

pub fn sum_invalid_ids_in_ranges(path: &PathBuf) -> i64 {
    let mut reader = buffered_reader(path).unwrap();
    let mut invalid_ids_sum: i64 = 0;
    let mut pending_bytes: Vec<u8> = Vec::new();

    loop {
        let buffer = reader.fill_buf().unwrap();

        if buffer.is_empty() {
            if !pending_bytes.is_empty() {
                let chunk = std::str::from_utf8(&pending_bytes).unwrap();
                process_chunk(chunk, &mut invalid_ids_sum);
            }
            break;
        }

        match buffer.iter().rposition(|&b| b == b',') {
            Some(last_comma_idx) => {
                if pending_bytes.is_empty() {
                    let chunk = std::str::from_utf8(&buffer[..=last_comma_idx]).unwrap();
                    process_chunk(chunk, &mut invalid_ids_sum);
                } else {
                    pending_bytes.extend_from_slice(&buffer[..=last_comma_idx]);
                    let chunk = std::str::from_utf8(&pending_bytes).unwrap();
                    process_chunk(chunk, &mut invalid_ids_sum);
                    pending_bytes.clear();
                }
                reader.consume(last_comma_idx + 1);
            }
            None => {
                pending_bytes.extend_from_slice(buffer);
                let consumed = buffer.len();
                reader.consume(consumed);
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

    mod invalid_ids_repeated_sequence {

    }

}