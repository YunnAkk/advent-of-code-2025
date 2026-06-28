mod days;

fn main() {
    let result = days::day01::count_dial_zero_hits(1, 50).unwrap();
    println!("{result}");
}
