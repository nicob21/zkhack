use test_puzzle::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    println!("My name is Nico");
}
