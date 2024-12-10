mod visuals;

fn main() {
    // program being (clear + banner)
    visuals::clear_terminal();
    visuals::print_banner();

    // some text
    println!();
    println!("Welcome to our Advent Puzzles!!!");
    println!();
}
