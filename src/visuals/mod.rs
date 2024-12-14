// declare files in this module (folder)
mod banners;
// using struct type from puzzles mod
use crate::puzzles::{AdventDay, Solve};

// self explanatory
pub fn clear_terminal() {
    // importing Command
    use std::process::Command;

    // calling command (clear to unix like || cls to windows)
    let _ = Command::new("clear")
        .status()
        .or_else(|_| Command::new("cls").status());
}

// return color escape based on ansi expected values
pub fn get_escape(font_style: i32, foreground: i32, background: i32) -> String {
    // start result with reset escape
    let mut final_escape: String = String::from("\x1b[0m");

    // collect all possible values for font, foreground and background
    let font_escope: Vec<i32> = (0..8).filter(|x| *x != 2 || *x != 5 || *x != 6).collect();
    let foreground_escope: Vec<i32> = (30..38).collect();
    let background_escope: Vec<i32> = (40..48).collect();

    // set especific argument to 0 means 'no apply', so
    // if the argument (foreach) isn't 0, push argument to final_escape (if in escope, else
    // default value)
    if font_style != 0 {
        final_escape.push_str(&format!(
            "\x1b[{}m",
            match font_escope.contains(&font_style) {
                true => font_style,
                _ => 4,
            }
        ));
    }
    if foreground != 0 {
        final_escape.push_str(&format!(
            "\x1b[{}m",
            match foreground_escope.contains(&foreground) {
                true => foreground,
                _ => 33,
            }
        ));
    }
    if background != 0 {
        final_escape.push_str(&format!(
            "\x1b[{}m",
            match background_escope.contains(&background) {
                true => background,
                _ => 4,
            }
        ));
    }

    final_escape
}

// just calls another function, but is less verbose
pub fn print_banner(banner_colors: (String, String)) {
    // local values that we'll use
    use banners::{print_merged, BELLOW_TAG, RUSTVENT_OF_CODE_TITLE};

    // calling print function passing these values
    print_merged(
        &RUSTVENT_OF_CODE_TITLE,            // title banner
        BELLOW_TAG,                         // bellow tag
        (banner_colors.0, banner_colors.1), // escapes
        (2, 1),                             // banner gaps
        (60, 0),                            // tag gaps
    );
}

// special printer to be used within solves function
pub fn solve_print(content: &str, new_line: bool) {
    // split content if new lines
    let contents: Vec<String> = content.split("\n").fold(Vec::<String>::new(), |mut a, b| {
        a.push(b.to_string());
        a
    });

    // for each line
    for line in contents {
        // print arrow + line
        print!("  {}>{}  ", get_escape(1, 32, 0), get_escape(0, 0, 0));
        print!("{}{}", line, if new_line { "\n" } else { "" });
    }
}

// self explanatory
pub fn print_advent_table(advents: &[AdventDay]) {
    // table header
    println!("  +---------+-------------------------+----------------------------+");
    println!("  |         |        Problems         |                            |");
    println!("  |  Days   +------------+------------+            Path            |");
    println!("  |         |   part 1   |   part 2   |                            |");
    println!("  +---------+------------+------------+----------------------------+");

    // foreach day
    for day in advents {
        // getting nth + solves bool
        let d_number: u8 = day.get_day_number();
        let solves: (bool, bool) = day.solves_as_bool();

        // printing day number
        print!(
            "  | Day {}  | ",
            match d_number {
                x if x < 10 => format!(" {}", x),
                _ => d_number.to_string(),
            }
        );

        // printting each solve status
        print!(
            "{} | {}",
            if solves.0 {
                format!("{} SOLVED \x1b[0m  ", get_escape(1, 37, 42))
            } else {
                format!("{} UNSOLVED \x1b[0m", get_escape(1, 37, 41))
            },
            if solves.1 {
                format!("{} SOLVED \x1b[0m  ", get_escape(1, 37, 42))
            } else {
                format!("{} UNSOLVED \x1b[0m", get_escape(1, 37, 41))
            }
        );

        // printing solve path
        print!(" | ");
        print!(
            "{}src/puzzles/day{}{}",
            get_escape(0, 33, 0),
            match d_number {
                x if x < 10 => format!("0{}", x),
                day => day.to_string(),
            },
            get_escape(0, 0, 0)
        );
        println!("          |");
    }

    // bottom border
    println!("  +---------+-------------------------+----------------------------+");
}

// self explanatory
pub fn print_about() {
    println!("  Will be implemented soon");
}

// print chall txt content
pub fn print_solve_content(solve: Option<&Solve>) {
    use banners::ROW_MAX_WIDTH;

    // getting values
    let (title, content, _) = solve.unwrap().get_challenge_values();

    // title + escape
    println!("\n");
    println!(
        "  {}",
        title
            .unwrap()
            .replace(" D", &{
                let mut d = get_escape(1, 31, 0);
                d.push('D');
                d
            })
            .replace(": ", &{
                let mut c = String::from(": ");
                c.push_str(&get_escape(0, 0, 0));
                c
            })
    );

    println!();

    // separating each row and entire content into new vectors
    let mut content_by_row: Vec<String> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();

    // iterate through each content line
    for row in content.unwrap().split("\n") {
        // iterate through each word
        for word in row.split(" ") {
            // push to current row + test (current row word count > max width)
            current_row.push(word.to_string());
            if current_row.iter().fold(0, |mut a, b| {
                a += b.len();
                a
            }) > ROW_MAX_WIDTH as usize
            {
                content_by_row.push(current_row.clone().join(" "));
                current_row.clear();
            }
        }
        content_by_row.push(current_row.clone().join(" "));
        current_row.clear();
    }

    // print each rearranged line
    for line in content_by_row {
        println!("  {}", line);
    }
}

// self explanatory
pub fn hide_marker() {
    use std::io::{self, Write};

    print!("\x1b[?25l");
    io::stdout().flush().unwrap();
}

// self explanatory
pub fn show_marker() {
    use std::io::{self, Write};

    print!("\x1b[?25h");
    io::stdout().flush().unwrap();
}

// simulates old videogames text
pub fn print_animated(content: &str) {
    // import all necessaries libs/mods
    use std::io::{self, Write};
    use std::thread;
    use std::time::Duration;

    io::stdout().flush().unwrap();

    // do sleep print
    for c in content.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    println!();
}
