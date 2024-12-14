// importing our modules and types
mod puzzles;
use puzzles::Solve;
mod visuals;
use getch_rs::{Getch, Key};
use std::collections::HashMap;
use visuals as vs;

// enum of possible menu inputs (kinda useless > I'm using literal comparations a lot)
enum MenuInput {
    GoToProblem,
    About,
    Quit,
}

// searching for enum value based on given input
fn find_input(reference: &HashMap<(Key, Key), MenuInput>, to_find: Key) -> Option<&MenuInput> {
    // foreach pair in our reference
    for (keys, value) in reference.iter() {
        // mach searching
        match to_find {
            x if x == keys.0 => return Some(value),
            x if x == keys.1 => return Some(value),
            _ => continue,
        }
    }
    // if not found
    None
}

// personal getch function
fn personal_getch(holder: &Getch) -> Option<Key> {
    // it will return an option instead Result<Key, _>
    // easy to handle
    match holder.getch() {
        Ok(x) => Some(x),
        _ => None,
    }
}

// personal parser (str -> i32) function
fn personal_parser(value: &str) -> Option<i32> {
    // returns an option of i32, so, i can handle parsers with no worries about panic!() on force
    // unwrap()

    // iterate in each value char
    for c in value.chars() {
        // if isn't digit, return None
        if !c.is_ascii_digit() {
            return None;
        }
    }

    // else, force unwrap() (every char is a digit, so... no errors coming)
    Some(value.parse::<i32>().unwrap())
}

// personal input function. Works linke Python's input
fn personal_input() -> String {
    // imported mods/libs
    use std::io::{self, Write};
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;

    // by clear and print on terminal, some unexpected behaviors occurred, such as:
    // - no printing when needs to print
    // - printing when doesn't asked to print
    //
    // probably because multiples io::stdout().flush().unwrap()
    //
    // so, I've been made this function to handle this bad behavior and it works. That's all, folks!

    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut final_result: String = String::new();

    // for each key you press
    for key in stdin.keys() {
        // match it
        match key.unwrap() {
            // if Enter, terminate
            Key::Char('\n') => break,
            // if a printable char
            Key::Char(c) => {
                // print into terminal + flush
                print!("{}", c);
                stdout.flush().unwrap();
                // push to final result
                final_result.push(c);
            }
            // if Backspace
            Key::Backspace => {
                // if result have +1 char => print back
                if !final_result.is_empty() {
                    print!("\x08 \x08");
                }
                // pop from result + flush
                let _ = final_result.pop();
                stdout.flush().unwrap();
            }
            // else, continue loop
            _ => {}
        }
    }
    // break one line + return result
    println!();
    final_result
}

fn main() {
    // hidding cursor
    vs::hide_marker();

    // start Getch var, alert text, menu options, solve options
    let menu_input: Getch = Getch::new();
    let mut alert: String = String::new();
    let options = [
        (" G ", "to go to solve"),
        (" A ", "to print about text"),
        (" Q ", "to quit program"),
    ];
    let solve_options = [(" R ", "to run solve"), (" ESC ", "to go back")];

    // start input_maps + insert on it
    let mut input_maps: HashMap<(Key, Key), MenuInput> = HashMap::new();
    input_maps.insert((Key::Char('G'), Key::Char('g')), MenuInput::GoToProblem);
    input_maps.insert((Key::Char('A'), Key::Char('a')), MenuInput::About);
    input_maps.insert((Key::Char('Q'), Key::Char('q')), MenuInput::Quit);

    // getting AdventDays collection (explained at puzzles/mod.rs)
    let advs: Vec<puzzles::AdventDay> = puzzles::gen_advent_days();

    // turning each option into a printable string (i can use a impl to do this, too)
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

    // turn solve options into printable
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

    // infinite menu loop while no breaks
    loop {
        // default escapes
        let reset_escape: String = vs::get_escape(0, 0, 0);
        let bold_white_blue: String = vs::get_escape(1, 37, 44);
        let bold_yellow_none: String = vs::get_escape(1, 33, 0);
        let bold_cyan_none: String = vs::get_escape(1, 36, 0);

        // clear + print banner
        vs::clear_terminal();
        vs::print_banner((bold_yellow_none, bold_cyan_none));
        println!();
        println!(
            "  Welcome to our {} Advent Puzzles {}!!!",
            bold_white_blue, reset_escape
        );
        println!();

        // print table with our advents puzzles
        vs::print_advent_table(&advs);
        println!();

        // ask for option + print alert (print nothing if alert is empty String)
        println!(
            "  Chose an option: {}{}{}",
            vs::get_escape(1, 31, 0),
            alert,
            vs::get_escape(0, 0, 0)
        );

        // alert already printed, so... clear it
        alert.clear();
        println!();

        // printing menu options
        println!("    {}", option_display);
        println!();

        // cath key
        let catch: Option<Key> = personal_getch(&menu_input);

        // if none (Err), redo loop
        if catch.is_none() {
            continue;
        }

        // searching for Menu Input by give Key
        let opt: Option<&MenuInput> = find_input(&input_maps, catch.unwrap());

        match opt {
            // if None (not found)
            None => continue,
            // if quit
            Some(MenuInput::Quit) => break,
            // if about
            Some(MenuInput::About) => {
                // clear + print about
                vs::clear_terminal();
                vs::print_about();

                // await response
                println!("  Press anything to continue...\n");
                let _ = menu_input.getch();

                // redo loop
                continue;
            }
            // if going to problem
            Some(MenuInput::GoToProblem) => {
                // show prompt
                vs::print_animated("  Put here an valid Advent Day followed by the target solve, like => 4 2 means: Day 4, part 2");
                print!("  > ");

                // show cursor
                vs::show_marker();

                // get input + hide cursor + trim response
                let mut response = personal_input();
                vs::hide_marker();
                response = response.trim().to_string();

                // turn response into a vec of 2 ints (expected)
                let parsed_search: Vec<Option<i32>> =
                    response
                        .split_whitespace()
                        .fold(Vec::<Option<i32>>::new(), |mut a, b| {
                            a.push(personal_parser(b));
                            a
                        });

                // match result
                match parsed_search {
                    // expected 2 elements
                    x if x.len() != 2 => {
                        alert.push_str("Invalid search input (day / solve)");
                        continue;
                    }
                    // expected a int day
                    x if x[0].is_none() => {
                        alert.push_str("Invalid day input (numbers only)");
                        continue;
                    }
                    // expected a int solve
                    x if x[1].is_none() => {
                        alert.push_str("Invalid solve input (numbers only)");
                        continue;
                    }
                    // only days between 1 <=> 25
                    x if !(1..=25).contains(&x[0].unwrap()) => {
                        alert.push_str("Target day is out of scope (1 - 25)");
                        continue;
                    }
                    // only solves between 1 <=> 2
                    x if !(1..=2).contains(&x[1].unwrap()) => {
                        alert.push_str("Target solve is out of scope (1 - 2)");
                        continue;
                    }
                    // if all true
                    v => {
                        // destructure elements and searchin for solve
                        vs::clear_terminal();
                        let (day, problem) = (v[0].unwrap() - 1, v[1].unwrap() - 1);
                        let found_solve: Option<&Solve> = advs[day as usize].get_solve(problem);

                        // if solve is None (not found)
                        if found_solve.is_none() {
                            // update alert
                            alert.push_str(&format!(
                                "The target solve {}couldn't be found{} (probably unsolved/file not found)",
                                vs::get_escape(1, 31, 0),
                                vs::get_escape(0, 0, 0)
                            ));
                            continue;
                        }

                        // printing challenge content
                        vs::print_solve_content(found_solve);

                        // prompt awaiting
                        println!("\n  ---\n");
                        println!("  You can scroll the terminal to see the entire text!\n\n");

                        // print options
                        println!("  {}", printable_options);

                        // loop
                        loop {
                            // expecting RUN or ESC
                            match menu_input.getch() {
                                // if RUN
                                Ok(Key::Char(c)) if c == 'r' || c == 'R' => {
                                    // clear terminal + (print tile | get input)
                                    vs::clear_terminal();
                                    let (title, _, input) =
                                        found_solve.unwrap().get_challenge_values();
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
                                            holder[0] = format!(
                                                "{}{}",
                                                vs::get_escape(0, 32, 0),
                                                holder[0]
                                            );

                                            holder.join("")
                                        },
                                        vs::get_escape(0, 0, 0)
                                    );
                                    println!();

                                    // run solve
                                    found_solve.unwrap().run_solve(input.unwrap());

                                    // expecting ESC to continue
                                    println!();
                                    println!(
                                        "  Press {} ESC {} to continue",
                                        vs::get_escape(1, 37, 41),
                                        vs::get_escape(0, 0, 0)
                                    );

                                    // probably exists some fancy way to do this (i don't know how)
                                    loop {
                                        match menu_input.getch() {
                                            Ok(Key::Esc) => break,
                                            _ => (),
                                        }
                                    }
                                    // go out solve loop after run
                                    break;
                                }
                                // go out solve loop with no run
                                Ok(Key::Esc) => {
                                    break;
                                }
                                // else, do nothing
                                _ => (),
                            }
                        }
                    }
                }
            }
        };
    }

    // bye text
    vs::clear_terminal();
    println!();
    println!(
        "  {} Bye {} ^^ {}",
        vs::get_escape(1, 37, 42),
        vs::get_escape(1, 37, 41),
        vs::get_escape(0, 0, 0)
    );
    println!();
    vs::show_marker();
}
