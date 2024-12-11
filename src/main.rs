mod visuals;

fn main() {
    let bold_yellow: String = visuals::get_escape(1, 33, 0);
    let bold_cyan: String = visuals::get_escape(1, 36, 0);

    // program begin (clear + banner)
    visuals::clear_terminal();
    visuals::print_banner((bold_yellow, bold_cyan));

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
