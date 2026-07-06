use crate::days::utils::buffered_reader;
use std::io::BufRead;
use std::path::PathBuf;

fn process_chunk(input: &str, results: &mut Vec<i64>) {
    let ranges = input.split(',');

    for range in ranges {
        if range.is_empty() {
            continue;
        }

        let mut parts = range.split('-');

        // TODO: remove unwrap
        let start_num_view = parts.next().unwrap();
        let end_num_view = parts.next().unwrap();
        let mut current_num_view = String::from(start_num_view);
        let mut current_num = current_num_view.parse::<i64>().unwrap();
        let end_num = end_num_view.parse::<i64>().unwrap();

        while current_num <= end_num {
            let current_num_len = current_num_view.len();

            if (current_num_len % 2) == 0 {
                let mid = current_num_len / 2;
                let left_view = &current_num_view[..mid];
                let right_view = &current_num_view[mid..];

                let left_num = left_view.parse::<i64>().unwrap();
                let right_num = right_view.parse::<i64>().unwrap();
                let base = 10_i64.pow(mid as u32);

                if left_num > right_num {
                    let diff = left_num - right_num;
                    current_num += diff;
                    current_num_view = current_num.to_string();
                    continue;
                } else if left_num == right_num {
                    results.push(current_num);
                    current_num += base;
                    current_num_view = current_num.to_string();
                    continue;
                } else if left_num < right_num {
                    let left_num_carry = left_num + 1;
                    let diff = base - right_num;
                    current_num += diff;
                    current_num += left_num_carry;
                    current_num_view = current_num.to_string();
                    continue;
                }
            } else {
                let shift_base = 10_i64.pow(current_num_len as u32);
                let diff = shift_base - current_num;
                current_num += diff;
                current_num_view = current_num.to_string();
            }
        }
    }
}

pub fn func_day2(path: &PathBuf) {
    let mut reader = buffered_reader(path).unwrap();
    let mut results: Vec<i64> = Vec::new();
    let mut leftover: Vec<u8> = Vec::new();

    loop {
        let buffer = reader.fill_buf().unwrap();

        if buffer.is_empty() {
            if !leftover.is_empty() {
                let chunk = std::str::from_utf8(&leftover).unwrap();
                process_chunk(chunk, &mut results);
            }
            break;
        }

        match buffer.iter().rposition(|&b| b == b',') {
            Some(last_comma_idx) => {
                if leftover.is_empty() {
                    let chunk = std::str::from_utf8(&buffer[..=last_comma_idx]).unwrap();
                    process_chunk(chunk, &mut results);
                } else {
                    leftover.extend_from_slice(&buffer[..=last_comma_idx]);
                    let chunk = std::str::from_utf8(&leftover).unwrap();
                    process_chunk(chunk, &mut results);
                    leftover.clear();
                }
                reader.consume(last_comma_idx + 1);
            }
            None => {
                leftover.extend_from_slice(buffer);
                let consumed = buffer.len();
                reader.consume(consumed);
            }
        }
    }

    println!("{:?}", results);
    let final_res: i64 = results.iter().sum();
    println!("{final_res}");
}
