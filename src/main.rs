mod visuals;

fn main() {
    // program begin (clear + banner)
    visuals::clear_terminal();
    visuals::print_banner();

    let background_red: String = visuals::get_escape(1, 30, 41);
    let reset: String = visuals::get_escape(0, 0, 0);

    // some text
    println!();
    println!(
        "Welcome to our {} Advent Puzzles {} !!!",
        background_red, reset
    );
    println!();
}
