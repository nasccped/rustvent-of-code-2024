pub fn try_read_file(file_path: &str) -> Option<Vec<String>> {
    use std::fs::File;
    use std::io::Read;

    let file_holder = File::open(file_path);

    if file_holder.is_err() {
        return None;
    }

    let mut file_holder = file_holder.unwrap();
    let mut content_holder: String = String::new();

    if file_holder.read_to_string(&mut content_holder).is_ok() {
        Some(
            content_holder
                .split("\n")
                .map(|row| row.to_string())
                .collect(),
        )
    } else {
        None
    }
}

use getch_rs::{Getch, Key};

pub fn get_user_key(target: &mut Key) {
    let getcher = Getch::new();

    loop {
        if let Ok(val) = getcher.getch() {
            *target = val;
            return;
        }
    }
}

pub fn about() {
    let project_repo = "https://github.com/nasccped/rustvent-of-code-2024";
    let reset = "\x1b[0m";
    let bold_def_def = "\x1b[1m";
    let bold_red_def = "\x1b[1;31m";
    let bold_green_def = "\x1b[1;32m";
    let bold_yellow_def = "\x1b[1;33m";
    let bold_yellow_red = "\x1b[1;33;41m";
    let bold_blue_def = "\x1b[1;34m";
    let bold_white_def = "\x1b[1;37m";
    let ital_def_def = "\x1b[3m";
    let bold_orange_def = "\x1b[1;38;2;255;130;0m";

    println!(
        "  Welcome to {}Rustvent Of Code - 2024{}",
        bold_yellow_def, reset
    );
    println!();
    print!(
        "  This program was built by me in an attempt to learn the {}",
        bold_def_def
    );
    let rust_lang = "Rust Language!";
    let (r, mut g, mut b) = (255, 130, 0);

    for l in rust_lang.chars() {
        print!("\x1b[38;2;{};{};{}m{}", r, g, b, l);
        g += 8;
        b += 18;
    }
    println!("{}", reset);
    println!();

    println!("  {}How to use:{}", bold_yellow_def, reset);
    println!(
        "    {}1.{} You can use the {}Vim{} Keys to navigate, such as h, j, k, l (And also {}Shift{} + they same).",
        bold_yellow_def, reset, bold_green_def, reset, bold_blue_def, reset
    );
    println!(
        "    {}2.{} When pressing {}Enter{} on a select puzzle (index by {}Day{} {}<Up, Down>{} and {}Part{} {}<Left, Right>{})",
        bold_yellow_def, reset, bold_blue_def, reset, bold_blue_def, reset, ital_def_def, reset, bold_blue_def, reset, ital_def_def, reset
    );
    println!(
        "       the terminal will display his scenario (if less than 30 rows - clear terminal issue {}.zZ{}).",
        bold_blue_def, reset
    );
    println!(
        "    {}3.{} The scenario will be printed only if the puzzle is {}solved{} ({}red color{} mean unsolved/None value).",
        bold_yellow_def, reset, bold_green_def, reset, bold_red_def, reset
    );
    println!(
        "    {}4.{} {}Enter{} one more time will run the solve. {}ESC{} mean go back.",
        bold_yellow_def, reset, bold_blue_def, reset, bold_blue_def, reset
    );
    println!(
        "    {}5.{} If you get lost, there are some subs in the 'program footer' to guide you.",
        bold_yellow_def, reset
    );
    println!(
        "    {}6.{} Selecting {}About{} will display this screen. {}Quit{} will... well... quit.",
        bold_yellow_def, reset, bold_blue_def, reset, bold_blue_def, reset
    );
    println!();

    println!("  {}Notes:{}", bold_yellow_def, reset);
    print!(
        "    > I've already had contact with other programming languages {}(",
        bold_white_def
    );

    let prog_langs = ["Java", "Python", "C Lang", "Kotlin"];
    type CompilerIsArguing = ((i32, i32, i32), (i32, i32, i32));
    let rgbs: [CompilerIsArguing; 4] = [
        ((254, 138, 24), (80, 133, 188)),
        ((55, 109, 154), (252, 209, 66)),
        ((21, 115, 184), (132, 168, 184)),
        ((249, 141, 19), (126, 117, 228)),
    ];

    let mut cursed_langs: Vec<String> = Vec::new();
    for (word, (from, to)) in prog_langs.iter().zip(rgbs.iter()) {
        let r_ratio: i32 = (to.0 - from.0) / (word.len() - 1) as i32;
        let g_ratio: i32 = (to.1 - from.1) / (word.len() - 1) as i32;
        let b_ratio: i32 = (to.2 - from.2) / (word.len() - 1) as i32;
        let (mut r, mut g, mut b) = (from.0, from.1, from.2);
        let mut final_string = String::new();
        for c in word.chars() {
            final_string.push_str(&format!("\x1b[1;38;2;{};{};{}m", r, g, b));
            final_string.push(c);
            r += r_ratio;
            g += g_ratio;
            b += b_ratio;
        }
        cursed_langs.push(final_string);
    }

    println!(
        "{}{}){}",
        cursed_langs.join("\x1b[1;37m, "),
        bold_white_def,
        reset
    );

    println!(
        "      and I decided to give {}Rust{} a chance!",
        bold_orange_def, reset
    );

    println!(
        "    > I'm not a professional {}rustacean{}/{}programmer{} (for now). It took me approximately 1 month",
        bold_orange_def, reset, bold_yellow_def, reset
    );
    println!("      to produce the initial stage (tui + day1 + day2)");
    println!("    > I tried to do it as quickly as I could to avoid the {}challenge deadline{} (December) and test", bold_red_def, reset);
    println!("      the {}learning curve{}!", bold_green_def, reset);
    println!(
        "    > {}Perhaps the solution to an event is not the best or not correct :^({}",
        ital_def_def, reset
    );
    println!(
        "    > {}The language is {}11/10{}{}, by the way...{}",
        ital_def_def, bold_green_def, reset, ital_def_def, reset
    );
    println!();

    println!(
        "  {} Did you find a bug / wrong answer? {}",
        bold_yellow_red, reset
    );
    println!(
        "    > {}Consider openning an issue at {}{}{}",
        ital_def_def, bold_green_def, project_repo, reset
    );
}
