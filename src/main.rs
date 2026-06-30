mod days;

fn main() {
    let path = days::utils::get_path_from_root("inputs/day01/input.txt");
    let result = days::day01::count_dial_zero_hits(&path, 50).unwrap();
    println!("{result}");
}