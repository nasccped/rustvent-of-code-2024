mod visuals;

fn main() {
    // program begin (clear + banner)
    visuals::clear_terminal();
    visuals::print_banner();

    // some text
    println!();
    println!("Welcome to our Advent Puzzles!!!");
    println!();
}
