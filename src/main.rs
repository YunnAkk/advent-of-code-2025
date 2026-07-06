mod days;

fn main() {
    let day01_path = days::utils::get_path_from_root("inputs/day01/input.txt");
    let day02_path = days::utils::get_path_from_root("test_inputs/day02/aoc_example.txt");

    let day01_part1 = days::day01::count_dial_zero_hits(&day01_path, 50);
    let day01_part2 = days::day01::count_dial_zero_passes(&day01_path, 50);
    println!("==========================================");
    println!("Result for Day 1 Part 1: {:?}", day01_part1);
    println!("Result for Day 1 Part 1: {:?}", day01_part2);

    let day02_part1 = days::day02::sum_invalid_ids_in_ranges(&day02_path);
    println!("==========================================");
    println!("Result for Day 2 Part 2: {:?}", day02_part1);
}
