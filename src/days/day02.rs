use crate::days::utils;
use crate::days::utils::buffered_reader;
use std::io::BufRead;
use std::path::PathBuf;

pub fn func_day2(path: &PathBuf) {
    let mut reader = buffered_reader(path).unwrap();
	let mut results: Vec<i64> = Vec::new();

    loop {
        let buffer = reader.fill_buf().unwrap();
        if buffer.is_empty() {
            break;
        }

        if let Some(last_comma_idx) = buffer.iter().rposition(|&b| b == b',') {
            let raw_bytes = &buffer[..=last_comma_idx];

            let utf8_chunk = std::str::from_utf8(raw_bytes).expect("File contained invalid UTF-8");

			process_chunk(&utf8_chunk, &mut results);

			let consumed = raw_bytes.len();
			reader.consume(consumed);
        } else {
			// TODO: Because in the above let Some branch the buffer is only consumed up to the last comma in this branch we have to clear what's left of the buffer, for example it might be the very last range block and we also have to deal with the case where the file was not able to be read or rather that it was empty

			break;
		}
    }

	println!("{:?}", results)

	// TODO: Sum up every number from the Results vector
}

fn process_chunk(input: &str, results: &mut Vec<i64>) {
	let ranges = input.split(',');

	for range in ranges {
		if range.is_empty() { continue; }

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
					let diff = right_num - left_num;
					current_num += diff * base;
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
