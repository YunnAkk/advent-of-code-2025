use std::io::Read;

mod days;

fn main() {
    let path = days::utils::get_path_from_root("test_inputs/day02/aoc_example.txt");
    // let result = days::day01::count_dial_zero_hits(&path, 50).unwrap();

    days::day02::func_day2(&path);
}