mod days;
#[cfg(test)]
mod test_utils;

fn main() {
    let day01_path = days::utils::get_path_from_root("inputs/day01/input.txt");
    let day02_path = days::utils::get_path_from_root("inputs/day02/input.txt");
    let day03_path = days::utils::get_path_from_root("inputs/day03/input.txt");

    let day01_part1 = days::day01::count_dial_zero_hits(&day01_path, 50).unwrap();
    let day01_part2 = days::day01::count_dial_zero_passes(&day01_path, 50).unwrap();
    println!("==========================================");
    println!("Result for Day 1 Part 1: {:?}", day01_part1);
    println!("Result for Day 1 Part 1: {:?}", day01_part2);

    let day02_part1 = days::day02::sum_invalid_ids_in_ranges(&day02_path);
    let day02_part2 = days::day02::sum_repeating_invalid_ids_in_ranges(&day02_path);
    println!("==========================================");
    println!("Result for Day 2 Part 1: {:?}", day02_part1);
    println!("Result for Day 2 Part 2: {:?}", day02_part2);

    let day03_part1 = days::day03::calculate_two_digit_joltage(&day03_path);
    println!("==========================================");
    println!("Result for Day 3 Part 1: {:?}", day03_part1)
}
