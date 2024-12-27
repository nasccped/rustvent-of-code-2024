use crate::navigation::ProgramSection;
use crate::navigation::XYCoordinates;
use crate::solves;
use crate::utils;
use crate::visuals;

use getch_rs::Key;

pub fn run() {
    visuals::hide_cursor();

    visuals::clear_terminal();
    visuals::print_banner();
    println!();

    let main_menu_escope = vec![
        Key::Char('h'),
        Key::Char('j'),
        Key::Char('k'),
        Key::Char('l'),
        Key::Char('H'),
        Key::Char('J'),
        Key::Char('K'),
        Key::Char('L'),
        Key::Up,
        Key::Down,
        Key::Left,
        Key::Right,
        Key::Char('\r'),
        Key::Esc,
    ];

    let mut coordinates_holder = XYCoordinates::new((1, 26));
    let mut current_section = ProgramSection::MainMenu;
    let mut key_holder: Key;
    let advents = solves::get_all_solves();
    let mut clear_ratio: u16;
    let mut target_advent: Option<solves::Solve> = None;

    loop {
        key_holder = Key::EOF;

        match &current_section {
            ProgramSection::MainMenu => {
                visuals::print_advent_table(&coordinates_holder, &advents);
                clear_ratio = 30;

                println!();
                println!("  {}", "-".repeat(77));
                println!("  You can use \x1b[1;33mArrow\x1b[0m / \x1b[1;32mVim\x1b[0m [h, j, k, l] keys and \x1b[1;37mEnter\x1b[0m to navigate!");
            }
            ProgramSection::Solve((a, b)) => {
                let adv_holder = advents[*a as usize].clone();
                target_advent = if b == &0 { adv_holder.0 } else { adv_holder.1 };

                if target_advent.is_none() {
                    current_section = ProgramSection::MainMenu;
                    continue;
                }

                clear_ratio = target_advent.clone().unwrap().get_scenario().len() as u16 + (5);
                target_advent.clone().unwrap().show_data();

                println!();
                println!("  {}", "-".repeat(77));
                println!("  Press \x1b[1;30;47m ESC \x1b[0m to go back or \x1b[1;37;42m Enter \x1b[0m to run...");
            }
            ProgramSection::RunningSolve(sv) => {
                println!("  \x1b[1;32m{}\x1b[0m", sv.get_title());
                println!();

                sv.run();

                println!();
                println!("  {}", "-".repeat(77));
                println!("  Press \x1b[1;30;47m Anything \x1b[0m to continue.");

                clear_ratio = 6;
            }
            ProgramSection::About => {
                utils::about();

                println!();
                println!("  {}", "-".repeat(77));
                println!("  Press \x1b[1;30;47m Anything \x1b[0m to continue.");

                clear_ratio = 28;
            }
            ProgramSection::Quit => {
                break;
            }
        }

        while !main_menu_escope.contains(&key_holder) {
            utils::get_user_key(&mut key_holder);
        }

        match (&current_section, &key_holder, &target_advent) {
            (&ProgramSection::MainMenu, Key::Char('\r'), _) => match coordinates_holder.get_xy() {
                (_, 25) => {
                    current_section = ProgramSection::About;
                }
                (_, 26) => {
                    current_section = ProgramSection::Quit;
                }
                (x, y) => {
                    current_section = ProgramSection::Solve((y, x));
                }
            },
            (ProgramSection::Solve(_), Key::Esc, _) => {
                current_section = ProgramSection::MainMenu;
            }
            (ProgramSection::Solve(_), Key::Char('\r'), solve) => {
                current_section = ProgramSection::RunningSolve(solve.clone().unwrap())
            }
            (ProgramSection::RunningSolve(_), ..) => {
                current_section = ProgramSection::MainMenu;
            }
            (ProgramSection::About, ..) => {
                current_section = ProgramSection::MainMenu;
            }
            (_, a, _) => {
                coordinates_holder.update(a.clone());
            }
        }
        visuals::clear_lines(clear_ratio);
    }

    let xmas_tree: Vec<String> = [
        ["\x1b[1;33m", "       *       ", "", "", "", ""],
        ["\x1b[1;32m", "      ,-.      ", "", "", "", ""],
        ["\x1b[1;32m", "     ,_-_.     ", "", "", "", ""],
        ["\x1b[1;32m", "    ,-_-", "\x1b[1;33m", "O", "\x1b[1;32m", "-.    "],
        ["\x1b[1;32m", "   ,_-_-_-_.   ", "", "", "", ""],
        ["\x1b[1;32m", "  ,-_", "\x1b[1;31m", "o", "\x1b[1;32m", "_-_-_-.  "],
        ["\x1b[1;32m", " ,_-_-_", "\x1b[1;34m", "0", "\x1b[1;32m", "_-.-_. "],
        ["\x1b[1;32m", ".-_", "\x1b[1;35m", "^", "\x1b[1;32m", "_-_-_-_-_-."],
        ["\x1b[0m", "      [_]      ", "", "", "", ""],
    ].map(|colec| colec.join("").to_string()).to_vec();

    for row in xmas_tree {
        println!("  {}{}", " ".repeat(31), row);
    }

    println!();
    println!("  {:^78}", "Made by Nasccped (2024 - 2025). Take a look to the");
    println!("  {:^78}", "repo at: https://github.com/nasccped/rustvent-of-code-2024");
    println!();

    visuals::show_cursor();
}

/*


       *
      ,-.
     ,_-_.
    ,-_-O-.
   ,_-_-_-_.
  ,-_o_-_-_-.
 ,_-_-_0_-.-_.
.-_^_-_-_-_-_-.
      [_]
*/
