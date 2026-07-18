//! Gift Shop — Invalid IDs
//!
//! A gift shop database has invalid product IDs mixed into otherwise valid
//! ranges. The puzzle input is a comma separated list of ranges (e.g.
//! `11-22,95-115,998-1012`), each giving a first and last ID separated by a
//! dash. IDs never have leading zeroes.
//!
//!
//! # Part One
//! Sum invalid IDs which are defined as an even number with the left
//! half being equal to its right half
//!
//! # Part Two
//! Sum invalid IDs which are defined as a number made up of a repeating
//! pattern of numbers
use crate::days::utils::buffered_reader;
use std::io::BufRead;
use std::path::PathBuf;

/// Returns the number of base-10 digits in `num`.
///
/// Panics for negative values.
fn get_number_length(num: i64) -> u32 {
    match num.checked_ilog10() {
        Some(log) => log + 1,
        None if num == 0 => 1,
        _ => panic!("Negative number error"),
    }
}

/// Splits `bytes` on the first ASCII hyphen (`-`) into `(before, after)`.
///
/// Returns `None` if no hyphen is present. Does not validate that either
/// side is numeric; use [`parse_int`] on the results for that.
fn parse_range(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    if let Some(hyphen_idx) = bytes.iter().position(|&b| b == b'-') {
        let start_bytes = &bytes[..hyphen_idx];
        let end_bytes = &bytes[(hyphen_idx + 1)..];

        Some((start_bytes, end_bytes))
    } else {
        None
    }
}

/// Parses an `i64` from a byte slice, ignoring any ASCII whitespace.
///
/// Returns `None` if the slice contains a non-digit, non-whitespace byte,
/// or if it contains no digits at all (e.g. empty or whitespace-only input).
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

/// Sums all invalid IDs in `[start, end]`, where an ID is invalid if its
/// digit count is even and its left half is equivalent to its right half.
///
/// Uses a jump ahead strategy rather than checking every number, jumps ahead
/// on each iteration: to the next matching invalid ID candidate,
///
/// Panics if any number in the range would require 19 or more digits
/// (too large to safely represent as `i64`).
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

/// Breaks down a whole number into its individual digits and appends each
/// digit to a vector, most significant digit first.
///
/// No op for `num <= 0`, `digits` is left unchanged in that case.
fn separate_num_to_digits(mut num: i64, digits: &mut Vec<i64>) {
    if num <= 0 {
        return;
    }
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
}

/// Returns `true` if `pattern` with `pattern_len` digits is not itself
/// a repetition of some smaller digit block.
///
/// For example, `12` is primitive, but `1212` (a repeat of `12`) and `55`
/// (a repeat of `5`) are not. Used to avoid double counting IDs that are
/// periodic at multiple block lengths.
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

/// Sums all IDs in `[start, end]` that consist of a primitive digit pattern
/// repeated two or more times (e.g. `121212`, `9595`, `1111`).
///
/// Rather than enumerating every number in range, this iterates over every
/// valid (digit length `L`, pattern length `P`) pair, derives the closed form
/// multiplier for turning a `P` digit pattern into an `L` digit repeated ID,
/// and narrows the pattern search to just those producing a value inside
/// `[start, end]`.
///
/// Panics if `end` would require 19 or more digits (too large for `i64`).
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

/// Reads comma separated `start-end` ranges from the file at `path`,
/// applies a function `f` to each parsed `(start, end)` pair and sums the results.
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

/// Part 1 — sums all invalid IDs across every range in the file at `path`.
///
/// See [`sum_invalid_in_range`] for the invalidity rule applied to each range.
pub fn sum_invalid_ids_in_ranges(path: &PathBuf) -> i64 {
    sum_over_ranges_in_file(path, sum_invalid_in_range)
}

/// Part 2 — sums all invalid IDs formed by a repeated pattern across every
/// range in the file at `path`.
///
/// See [`sum_repeating_invalid_id_in_range`] for the invalidity rule applied
/// to each range.
pub fn sum_repeating_invalid_ids_in_ranges(path: &PathBuf) -> i64 {
    sum_over_ranges_in_file(path, sum_repeating_invalid_id_in_range)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::define_day_in_path;
    
    define_day_in_path!("02");

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
        fn i64_max() {
            assert_eq!(get_number_length(i64::MAX), 19);
        }

        #[test]
        #[should_panic]
        fn negative() {
            get_number_length(-1);
        }

        #[test]
        #[should_panic]
        fn i64_min() {
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

    mod num_to_digit {
        use super::*;

        #[test]
        fn single_digits() {
            let mut digits = Vec::new();
            for i in 1..=9 {
                digits.clear();
                separate_num_to_digits(i, &mut digits);
                assert_eq!(digits, vec![i])
            }
        }

        #[test]
        fn multi_digits() {
            let mut digits = Vec::new();
            separate_num_to_digits(123, &mut digits);
            assert_eq!(digits, vec![1, 2, 3]);
        }

        #[test]
        fn trailing_zeros() {
            let mut digits = Vec::new();
            separate_num_to_digits(100, &mut digits);
            assert_eq!(digits, vec![1, 0, 0]);
        }

        #[test]
        fn internal_zeros() {
            let mut digits = Vec::new();
            separate_num_to_digits(1024, &mut digits);
            assert_eq!(digits, vec![1, 0, 2, 4]);
        }

        #[test]
        fn repeated_digits() {
            let mut digits = Vec::new();
            separate_num_to_digits(111111, &mut digits);
            assert_eq!(digits, vec![1, 1, 1, 1, 1, 1]);
        }

        #[test]
        fn power_of_ten() {
            let mut digits = Vec::new();
            separate_num_to_digits(1000, &mut digits);
            assert_eq!(digits, vec![1, 0, 0, 0]);
        }

        #[test]
        fn i64_max() {
            let mut digits = Vec::new();
            separate_num_to_digits(i64::MAX, &mut digits);
            assert_eq!(
                digits,
                vec![9, 2, 2, 3, 3, 7, 2, 0, 3, 6, 8, 5, 4, 7, 7, 5, 8, 0, 7]
            );
        }

        #[test]
        fn zero_is_no_op() {
            let mut digits = Vec::new();
            separate_num_to_digits(0, &mut digits);
            assert!(digits.is_empty(), "Zero should not push any digits");
        }

        #[test]
        fn zero_leaves_existing_vec_unchanged() {
            let mut digits = vec![9, 9];
            separate_num_to_digits(0, &mut digits);
            assert_eq!(digits, vec![9, 9], "Zero must not modify existing vector");
        }

        #[test]
        fn negative_one_unchanged() {
            let mut digits = vec![4, 2];
            separate_num_to_digits(-1, &mut digits);
            assert_eq!(
                digits,
                vec![4, 2],
                "Negative numbers must not modify existing vector"
            );
        }

        #[test]
        fn i64_min_unchanged() {
            let mut digits = vec![7];
            separate_num_to_digits(i64::MIN, &mut digits);
            assert_eq!(digits, vec![7], "i64::MIN must not modify existing vector");
        }
    }

    mod primitive_pattern_detection {
        use super::*;

        #[test]
        fn single_digit_always_primitive() {
            for i in 0..=9 {
                assert!(is_primitive_pattern(i, 1))
            }
        }

        #[test]
        fn is_primitive_detected_correctly() {
            assert!(is_primitive_pattern(12, 2));
            assert!(is_primitive_pattern(121, 3));
            assert!(is_primitive_pattern(1122, 4));
            assert!(is_primitive_pattern(1213, 4));
            assert!(is_primitive_pattern(10203, 5));
            assert!(is_primitive_pattern(123456, 6));
            assert!(is_primitive_pattern(12345678, 8));
            assert!(is_primitive_pattern(123456789, 9));
        }

        #[test]
        fn is_not_primitive_detected_correctly() {
            assert!(!is_primitive_pattern(55, 2));
            assert!(!is_primitive_pattern(1212, 4));
            assert!(!is_primitive_pattern(123123, 6));
            assert!(!is_primitive_pattern(12341234, 8));
        }
    }

    mod invalid_in_range {
        use super::*;

        #[test]
        fn no_invalid_number() {
            assert_eq!(sum_invalid_in_range(1, 9), 0);
            assert_eq!(sum_invalid_in_range(10, 10), 0);
            assert_eq!(sum_invalid_in_range(100, 999), 0);
            assert_eq!(sum_invalid_in_range(100, 1000), 0);
            assert_eq!(sum_invalid_in_range(2134, 2221), 0);
        }

        #[test]
        fn single_invalid_number() {
            assert_eq!(sum_invalid_in_range(11, 11), 11);
            assert_eq!(sum_invalid_in_range(12, 30), 22);
            assert_eq!(sum_invalid_in_range(20, 30), 22);
            assert_eq!(sum_invalid_in_range(4848, 4848), 4848);
            assert_eq!(sum_invalid_in_range(123123, 123123), 123123);
            assert_eq!(sum_invalid_in_range(12341234, 12341234), 12341234);
        }

        #[test]
        fn multiple_invalid_numbers() {
            assert_eq!(sum_invalid_in_range(0, 100), 495);
            assert_eq!(sum_invalid_in_range(10, 99), 495);
            assert_eq!(sum_invalid_in_range(80, 100), 187);
            assert_eq!(sum_invalid_in_range(99, 1010), 1109);
            assert_eq!(sum_invalid_in_range(1000, 9999), 495405);
            assert_eq!(sum_invalid_in_range(100000, 999999), 495044550);
            assert_eq!(sum_invalid_in_range(10000000, 10010000), 10001000);
        }

        #[test]
        #[should_panic]
        fn panics_on_nineteen_digit_number() {
            sum_invalid_in_range(1000000000000000000, 1000000000000000000);
        }
    }

    mod invalid_ids_single_sequence {
        use super::*;

        #[test]
        fn aoc_example() {
            assert_eq!(sum_invalid_ids_in_ranges(&path_for("aoc_example.txt")), 1227775554);
        }

        #[test]
        fn left_equal_right() {
            assert_eq!(sum_invalid_ids_in_ranges(&path_for("left_equal_right.txt")), 48985);
        }

        #[test]
        fn left_greater() {
            assert_eq!(sum_invalid_ids_in_ranges(&path_for("left_greater_right.txt")), 4444);
        }

        #[test]
        fn left_lesser() {
            assert_eq!(sum_invalid_ids_in_ranges(&path_for("left_less_right.txt")), 2222);
        }

        #[test]
        fn no_invalid() {
            assert_eq!(sum_invalid_ids_in_ranges(&path_for("no_invalid.txt")), 0);
        }

        #[test]
        fn single_invalid() {
            assert_eq!(sum_invalid_ids_in_ranges(&path_for("single_invalid.txt")), 11);
        }

        #[test]
        fn many_sequences() {
            assert_eq!(sum_invalid_ids_in_ranges(&path_for("many_sequences.txt")), 24416684186);
        }
    }

    mod invalid_ids_repeated_sequence {
        use super::*;

        #[test]
        fn aoc_example() {
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path_for("aoc_example_repeating_ids.txt")), 4174379265);
        }

        #[test]
        fn left_equal_right() {
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path_for("left_equal_right.txt")), 48985);
        }

        #[test]
        fn left_greater() {
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path_for("left_greater_right.txt")), 4444);
        }

        #[test]
        fn left_lesser() {
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path_for("left_less_right.txt")), 2222);
        }

        #[test]
        fn no_invalid() {
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path_for("no_invalid.txt")), 0);
        }

        #[test]
        fn single_invalid() {
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path_for("single_invalid.txt")), 11);
        }

        #[test]
        fn many_sequences() {
            assert_eq!(sum_repeating_invalid_ids_in_ranges(&path_for("many_sequences.txt")), 25426801288);
        }
    }
}
