mod visuals;
use visuals as vs;

fn main() {
    let reset_escape: String = vs::get_escape(0, 0, 0);
    let bold_white_red: String = vs::get_escape(1, 37, 41);
    let bold_yellow_none: String = vs::get_escape(1, 33, 0);
    let bold_cyan_none: String = vs::get_escape(1, 36, 0);

    // program begin (clear + banner)
    vs::clear_terminal();
    vs::print_banner((bold_yellow_none, bold_cyan_none));

    // some text
    println!();
    vs::vprintln(&format!(
        "Welcome to our {} Advent Puzzles {}!!!",
        bold_white_red, reset_escape
    ));
    println!();

    vs::vprintln("So now...\nWe can print\nA bunch of lines!");

    vs::vprintln("\nxD\n");
}
