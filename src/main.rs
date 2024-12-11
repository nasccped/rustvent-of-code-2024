mod visuals;
use visuals as vs;

fn main() {
    let background_red: String = vs::get_escape(1, 30, 41);
    let reset: String = vs::get_escape(0, 0, 0);
    let bold_yellow: String = vs::get_escape(1, 33, 0);
    let bold_cyan: String = vs::get_escape(1, 36, 0);

    // program begin (clear + banner)
    vs::clear_terminal();
    vs::print_banner((bold_yellow, bold_cyan));

    // some text
    println!();
    vs::vprintln(&format!(
        "Welcome to our {} Advent Puzzles {} !!!",
        background_red, reset
    ));
    println!();

    vs::vprintln("So now...\nWe can print\nA bunch of lines!");

    vs::vprintln("\nxD\n");
}
