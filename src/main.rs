mod puzzles;
use puzzles::Solve;
mod visuals;
use getch_rs::{Getch, Key};
use std::collections::HashMap;
use std::io::Write;
use visuals as vs;

enum MenuInput {
    GoToProblem,
    About,
    Quit,
}

fn find_input(reference: &HashMap<(Key, Key), MenuInput>, to_find: Key) -> Option<&MenuInput> {
    for (keys, value) in reference.iter() {
        match to_find {
            x if x == keys.0 => return Some(value),
            x if x == keys.1 => return Some(value),
            _ => continue,
        }
    }
    None
}

fn personal_getch(holder: &Getch) -> Option<Key> {
    match holder.getch() {
        Ok(x) => Some(x),
        _ => None,
    }
}

fn personal_parser(value: &str) -> Option<i32> {
    for c in value.chars() {
        if !c.is_ascii_digit() {
            return None;
        }
    }

    Some(value.parse::<i32>().unwrap())
}

fn personal_input() -> String {
    use std::io::{self, Write};
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut result: String = String::new();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => break,
            Key::Char(c) => {
                print!("{}", c);
                stdout.flush().unwrap();
                result.push(c);
            }
            Key::Backspace => {
                let _ = result.pop();
                print!("\x08 \x08");
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }
    println!();
    result
}

fn main() {
    vs::hide_marker();

    let menu_input: Getch = Getch::new();
    let mut alert: String = String::new();
    let options = [
        (" G ", "to go to solve"),
        (" A ", "to print about text"),
        (" Q ", "to quit program"),
    ];

    let mut input_maps: HashMap<(Key, Key), MenuInput> = HashMap::new();
    input_maps.insert((Key::Char('G'), Key::Char('g')), MenuInput::GoToProblem);
    input_maps.insert((Key::Char('A'), Key::Char('a')), MenuInput::About);
    input_maps.insert((Key::Char('Q'), Key::Char('q')), MenuInput::Quit);

    loop {
        let reset_escape: String = vs::get_escape(0, 0, 0);
        let bold_white_blue: String = vs::get_escape(1, 37, 44);
        let bold_yellow_none: String = vs::get_escape(1, 33, 0);
        let bold_cyan_none: String = vs::get_escape(1, 36, 0);

        // program begin (clear + banner)
        vs::clear_terminal();
        vs::print_banner((bold_yellow_none, bold_cyan_none));

        // some text
        println!();
        println!(
            "  Welcome to our {} Advent Puzzles {}!!!",
            bold_white_blue, reset_escape
        );
        println!();

        let advs: Vec<puzzles::AdventDay> = puzzles::gen_advent_days();

        vs::print_advent_table(&advs);
        println!();

        println!(
            "  Chose an option: {}{}{}",
            vs::get_escape(1, 31, 0),
            alert,
            vs::get_escape(0, 0, 0)
        );

        alert.clear();
        println!();

        let option_display: String = options
            .iter()
            .map(|x| {
                let mut result: String = vs::get_escape(1, 37, 41);
                result.push_str(x.0);
                result.push_str(&vs::get_escape(0, 0, 0));
                result.push(' ');
                result.push_str(x.1);
                result
            })
            .collect::<Vec<_>>()
            .join(" | ");

        println!("    {}", option_display);
        println!();

        let catch = personal_getch(&menu_input);

        if catch.is_none() {
            continue;
        }

        let opt: Option<&MenuInput> = find_input(&input_maps, catch.unwrap());

        match opt {
            None => continue,
            Some(MenuInput::Quit) => break,
            Some(MenuInput::About) => {
                vs::clear_terminal();
                vs::print_about();

                println!("  Press anything to continue...\n");

                let _ = menu_input.getch();
                continue;
            }
            Some(MenuInput::GoToProblem) => {
                vs::print_animated("  Put here an valid Advent Day followed by the target solve, like => 4 2 means: Day 4, part 2");
                print!("  > ");
                vs::show_marker();
                std::io::stdout().flush().unwrap();

                let mut response = personal_input();
                vs::hide_marker();
                response = response.trim().to_string();

                let possible_search: Vec<Option<i32>> =
                    response
                        .split_whitespace()
                        .fold(Vec::<Option<i32>>::new(), |mut a, b| {
                            a.push(personal_parser(b));
                            a
                        });

                match possible_search {
                    x if x.len() != 2 => {
                        alert.push_str("Invalid search input (day / solve)");
                        continue;
                    }
                    x if x[0].is_none() => {
                        alert.push_str("Invalid day input (numbers only)");
                        continue;
                    }
                    x if x[1].is_none() => {
                        alert.push_str("Invalid solve input (numbers only)");
                        continue;
                    }
                    x if !(1..=25).contains(&x[0].unwrap()) => {
                        alert.push_str("Target day is out of scope (1 - 25)");
                        continue;
                    }
                    x if !(1..=2).contains(&x[1].unwrap()) => {
                        alert.push_str("Target solve is out of scope (1 - 2)");
                        continue;
                    }
                    v => {
                        vs::clear_terminal();
                        let (day, problem) = (v[0].unwrap() - 1, v[1].unwrap() - 1);
                        let found_solve: Option<&Solve> = advs[day as usize].get_solve(problem);

                        if found_solve.is_none() {
                            alert.push_str(&format!(
                                "The target solve {}couldn't be found{} (probably unsolved/file not found)",
                                vs::get_escape(1, 31, 0),
                                vs::get_escape(0, 0, 0)
                            ));
                            continue;
                        }

                        vs::print_solve_content(found_solve);
                        println!("\n  ---\n");
                        println!("  You can scroll the terminal to see the entire text!\n\n");

                        let solve_options = [(" R ", "to run solve"), (" Anything ", "to go back")];

                        let printable_options = solve_options
                            .iter()
                            .map(|x| {
                                let mut result: String = vs::get_escape(1, 37, 41);
                                result.push_str(x.0);
                                result.push_str(&vs::get_escape(0, 0, 0));
                                result.push(' ');
                                result.push_str(x.1);

                                result
                            })
                            .collect::<Vec<_>>()
                            .join(" | ");

                        println!("  {}", printable_options);

                        match menu_input.getch() {
                            Ok(Key::Char(c)) if c == 'r' || c == 'R' => {
                                vs::clear_terminal();
                                let (title, _, input) = found_solve.unwrap().get_challenge_values();
                                println!("\n");
                                println!(
                                    "   {}{}{}",
                                    vs::get_escape(0, 32, 0),
                                    {
                                        let mut holder: Vec<String> = title
                                            .unwrap()
                                            .split(":")
                                            .map(|x| x.to_string())
                                            .collect();

                                        holder[0].push_str(&vs::get_escape(0, 0, 0));
                                        holder[0] =
                                            format!("{}{}", vs::get_escape(0, 32, 0), holder[0]);

                                        holder.join("")
                                    },
                                    vs::get_escape(0, 0, 0)
                                );
                                println!();
                                found_solve.unwrap().run_solve(input.unwrap());

                                println!();
                                println!("  Press any key to continue.");
                                let _ = menu_input.getch();
                            }
                            _ => {}
                        }
                    }
                }
            }
        };
    }

    vs::show_marker();
}
