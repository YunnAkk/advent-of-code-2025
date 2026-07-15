use crate::days::utils::buffered_reader;
use std::io::BufRead;
use std::path::PathBuf;

fn get_number_length(num: i64) -> u32 {
    match num.checked_ilog10() {
        Some(log) => log + 1,
        None if num == 0 => 1,
        _ => panic!("Negative number error"),
    }
}

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
        let num_digits = get_number_length(current_num);

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

fn separate_num_to_digits(mut num: i64, digits: &mut Vec<i64>) {
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
}

fn is_primitive_pattern(pattern: i64, pattern_len: u32) -> bool {
    let mut digits: Vec<i64> = Vec::new();
    separate_num_to_digits(pattern, &mut digits);

    for sub_period in 1..pattern_len {
        if pattern_len % sub_period != 0 {
            continue;
        }

        let sub_period = sub_period as usize;

        let mut is_periodic = true;
        for i in 0..digits.len() {
            if digits[i] != digits[i % sub_period] {
                is_periodic = false;
                break;
            }
        }

        if is_periodic {
            return false;
        }
    }
    true
}

fn sum_repeating_invalid_id_in_range(start: i64, end: i64) -> i64 {
    let mut total_sum = 0;

    let start_len = get_number_length(start);
    let end_len = get_number_length(end);

    if end_len >= 19 {
        panic!("The number in the given range does not fit in an i64");
    }

    // l = number of digits in the IDs we're generating this iteration.
    for l in start_len..=end_len {
        // r_n represents the r^n in the geometric series formula S_n = a(r^n - 1)/(r - 1),
        // and numerator represents r^n - 1 which is the largest L-digit number.
        let r_n = 10_i64.pow(l);
        let numerator = r_n - 1;

        // p = length of one repetition of the pattern (the "block").
        // Two constraints on P:
        // - P must divide L evenly, so the pattern divides the ID with no leftover.
        // - "repeated at least twice" means k = L / P >= 2, or rather P <= L / 2.
        for p in 1..=(l / 2) {
            if l % p != 0 {
                continue;
            }

            // block represents r in the denominator from the geometric series formula.
            // The repunit_multiplier represents the multiplier to extend a number/pattern.
            // Every invalid ID of length L with pattern length P factors as
            // num = pattern * repunit_multiplier, where repunit_multiplier is the sum of
            // a geometric series with ratio r = 10^P and k = L/P terms.
            //
            // repunit_multiplier = 1 + 10^P + 10^(2P) + ... + 10^((k-1)P)
            //
            // A geometric series  1 + r + r^2 + ... + r^(n-1)  sums to (r^n - 1)/(r - 1).
            // Substituting r = 10^P and n = k, where k = terms = L/P:
            //
            // repunit_multiplier
            // = ((10^P)^k - 1) / (10^P - 1)
            // = (10^(kP) - 1) / (10^P - 1)
            // = (10^L - 1) / (10^P - 1)
            //
            // This equals the closed form sum of a geometric series.
            let block = 10_i64.pow(p);
            let repunit_multiplier = numerator / (block - 1);

            // Valid range of P digit patterns, from the smallest P digit
            // number up to block - 1, the largest P digit number
            let pattern_min = 10_i64.pow(p - 1);
            let pattern_max = block - 1;

            // Narrow the pattern range down to only those patterns whose resulting
            // repeated number (pattern * repunit_multiplier) actually falls within [start, end].
            // For the lower bound we want the smallest pattern whose product is >= start.
            // That is the smallest pattern satisfying p * M >= start, or rather p >= start / M.
            // where M is repunit_multiplier and p the pattern.
            // Since integer division truncates down, plain `start / M` could give a pattern
            // that's too small (p * M < start), so we round up instead via the
            // (start + M - 1) / M trick, adding M - 1 pushes any remainder over to the
            // next integer without affecting values that already divide evenly.
            // For the upper bound we want the largest pattern whose product is <= end.
            // That is the largest p satisfying  p * M <= end, or rather p <= end / M.
            // Here plain integer division is exactly what we want, `end / M` truncates
            // down, giving the largest p such that p * M <= end. No rounding trick
            // needed, since truncation and round down to satisfy <= are the same thing.
            let valid_min = std::cmp::max(
                pattern_min,
                (start + repunit_multiplier - 1) / repunit_multiplier,
            );
            let valid_max = std::cmp::min(pattern_max, end / repunit_multiplier);

            // No P digit pattern produces a number in [start, end] for this
            // L and P pairing, nothing to add, move on to next iteration.
            if valid_min > valid_max {
                continue;
            }

            for pattern in valid_min..=valid_max {
                // One ID can be periodic at several pattern lengths at once:
                // 111111 = "1" x6 = "11" x3 = "111" x2.
                // To count each invalid ID exactly once, accept P only when it
                // is the MINIMAL period of `pattern` i.e. when `pattern`
                // itself is not a repetition of some smaller block. This is
                // what is_primitive_pattern checks.
                if !is_primitive_pattern(pattern, p) {
                    continue;
                }

                // Reconstruct the full ID from its factorization.
                total_sum += pattern * repunit_multiplier;
            }
        }
    }

    total_sum
}

fn sum_over_ranges_in_file(path: &PathBuf, f: impl Fn(i64, i64) -> i64) -> i64 {
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
                invalid_ids_sum += f(start, end);
            }
        }
    }

    invalid_ids_sum
}

pub fn sum_invalid_ids_in_ranges(path: &PathBuf) -> i64 {
    sum_over_ranges_in_file(path, sum_invalid_in_range)
}

pub fn sum_repeating_invalid_ids_in_ranges(path: &PathBuf) -> i64 {
    sum_over_ranges_in_file(path, sum_repeating_invalid_id_in_range)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::utils::get_path_from_root;

    mod number_length {
        use super::*;

        #[test]
        fn zero_input() {
            assert_eq!(get_number_length(0), 1);
        }

        #[test]
        fn single_digits() {
            for i in 0..=9 {
                assert_eq!(get_number_length(i), 1);
            }
        }

        #[test]
        fn double_digits() {
            for i in 10..=99 {
                assert_eq!(get_number_length(i), 2);
            }
        }

        #[test]
        fn triple_digits() {
            for i in 100..=999 {
                assert_eq!(get_number_length(i), 3);
            }
        }

        #[test]
        fn maximum_positive() {
            assert_eq!(get_number_length(i64::MAX), 19);
        }

        #[test]
        #[should_panic]
        fn negative() {
            get_number_length(-1);
        }


        #[test]
        #[should_panic]
        fn maximum_negative() {
            get_number_length(i64::MIN);
        }






    }

    mod parse {
        use super::*;
        mod range {
            use super::*;

            #[test]
            fn basic_range() {
                assert_eq!(
                    parse_range(b"1872-2931"),
                    Some((&b"1872"[..], &b"2931"[..]))
                );
            }

            #[test]
            fn leading_zeros() {
                assert_eq!(parse_range(b"001-050"), Some((&b"001"[..], &b"050"[..])));
            }

            #[test]
            fn non_digit_chars() {
                assert_eq!(parse_range(b"abc-def"), Some((&b"abc"[..], &b"def"[..])));
            }

            #[test]
            fn empty_input() {
                assert_eq!(parse_range(b""), None);
            }

            #[test]
            fn single_number_no_hyphen() {
                assert_eq!(parse_range(b"42"), None);
            }

            #[test]
            fn whitespace_only_no_hyphen() {
                assert_eq!(parse_range(b"   "), None);
            }

            #[test]
            fn en_dash_is_not_ascii_hyphen() {
                // U+2013 en dash, not ASCII 0x2D
                let input: &[u8] = "10\u{2013}20".as_bytes();
                assert_eq!(parse_range(input), None);
            }

            #[test]
            fn em_dash_is_not_ascii_hyphen() {
                // U+2014 em dash, not ASCII 0x2D
                let input: &[u8] = "10\u{2014}20".as_bytes();
                assert_eq!(parse_range(input), None);
            }
        }

        mod int {
            use super::*;

            #[test]
            fn single_digits() {
                for digit in 0..9u8 {
                    let input = &[b'0' + digit];
                    let expected = digit as i64;
                    assert_eq!(
                        parse_int(input),
                        Some(expected),
                        "parse_int({:?}) should return Some({})",
                        input,
                        expected,
                    )
                }
            }

            #[test]
            fn multi_digit() {
                assert_eq!(parse_int(b"123456789"), Some(123456789));
            }

            #[test]
            fn leading_zeros() {
                assert_eq!(parse_int(b"0042"), Some(42));
            }

            #[test]
            fn leading_whitespace() {
                assert_eq!(parse_int(b" 99"), Some(99));
            }

            #[test]
            fn trailing_whitespace() {
                assert_eq!(parse_int(b"99 "), Some(99));
            }

            #[test]
            fn whitespace_between_digits() {
                assert_eq!(parse_int(b"1 2 3"), Some(123));
            }

            #[test]
            fn i64_max() {
                assert_eq!(parse_int(b"9223372036854775807"), Some(9223372036854775807));
            }

            #[test]
            fn invalid_sequence_returns_none() {
                assert_eq!(parse_int(b"abc"), None);
            }

            #[test]
            fn empty_slice_returns_none() {
                assert_eq!(parse_int(b""), None);
            }

            #[test]
            fn whitespace_only_returns_none() {
                assert_eq!(parse_int(b"   "), None);
            }
        }
    }

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
        use super::*;

        #[test]
        fn aoc_example() {
            let path = get_path_from_root("test_inputs/day02/aoc_example_repeating_ids.txt");
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path), 4174379265);
        }
    }
}
