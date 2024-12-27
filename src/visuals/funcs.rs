use crate::navigation::XYCoordinates;
use crate::solves::Solve;

pub fn hide_cursor() {
    use termion::cursor;
    print!("{}", cursor::Hide);
}

pub fn show_cursor() {
    use termion::cursor;
    print!("{}", cursor::Show);
}

pub fn clear_terminal() {
    use std::process::Command;

    let _ = Command::new("cls")
        .status()
        .or_else(|_| Command::new("clear").status());
}

pub fn print_advent_table(cur_coord: &XYCoordinates, advents: &[(Option<Solve>, Option<Solve>)]) {
    let cur_x = cur_coord.get_xy().0 + 1;
    let cur_y = cur_coord.get_xy().1;

    for i in 0..(advents.len() + 2) {
        let row: (Option<Solve>, Option<Solve>) = match i {
            a if a >= advents.len() => (None, None),
            _ => advents[i].clone(),
        };

        let day_number = match i + 1 {
            val if val < 10 => format!(" {}", val),
            val => val.to_string(),
        };

        let mut string = String::new();

        match (i, cur_y as usize) {
            (25, 25) => {
                string.push_str("\x1b[1;33;41m About");
                string.push_str(&(" ".repeat(71)));
                string.push_str("\x1b[0m");
            }
            (25, _) => {
                string.push_str("\x1b[1m About");
                string.push_str(&(" ".repeat(71)));
            }
            (26, 26) => {
                string.push_str("\x1b[1;33;41m Quit");
                string.push_str(&(" ".repeat(72)));
                string.push_str("\x1b[0m");
            }
            (26, _) => {
                string.push_str("\x1b[1m Quit");
                string.push_str(&(" ".repeat(72)));
            }
            (a, b) if a == b => {
                string.push_str("\x1b[1;37;44m Day");
                string.push_str(&day_number);
                string.push_str(" | ");

                match (cur_x, &row.0, &row.1) {
                    (1, a, _) if a.is_some() => {
                        string.push_str("\x1b[1;37;42m part1 ");
                        string.push_str("\x1b[1;37;44m part2  | ");
                    }
                    (1, a, _) if a.is_none() => {
                        string.push_str("\x1b[1;37;41m part1 ");
                        string.push_str("\x1b[1;37;44m part2  | ");
                    }
                    (2, _, b) if b.is_some() => {
                        string.push_str(" part1 ");
                        string.push_str("\x1b[1;37;42m part2 \x1b[37;44m | ");
                    }
                    _ => {
                        string.push_str(" part1 ");
                        string.push_str("\x1b[1;37;41m part2 \x1b[37;44m | ");
                    }
                }

                let title: &str = match (&row.0, &row.1) {
                    (None, None) => "---",
                    (Some(a), _) => &a.get_title(),
                    (_, Some(b)) => &b.get_title(),
                };
                let t_len: usize = title.len();
                string.push_str(title);
                string.push_str(&(" ".repeat(52 - t_len)));
                string.push_str("\x1b[0m");
            }
            _ => {
                string.push_str("\x1b[1m Day");
                string.push_str(&day_number);
                string.push_str(" | ");
                string.push_str(match row.0 {
                    None => "\x1b[1;31m part1 ",
                    _ => "\x1b[1;32m part1 ",
                });
                string.push_str(match row.1 {
                    None => "\x1b[1;31m part2 ",
                    _ => "\x1b[1;32m part2 ",
                });
                string.push_str("\x1b[0m");
                string.push_str("\x1b[1m | ");

                if let Some(sv) = row.0 {
                    string.push_str(&sv.get_title());
                } else if let Some(sv) = row.1 {
                    string.push_str(&sv.get_title());
                } else {
                    string.push_str("---");
                }
            }
        }
        string.push_str("\x1b[0m");

        println!("  {}", string);
    }
}

pub fn clear_lines(n: u16) {
    use std::io::{self, Write};
    use termion::{clear, cursor};

    io::stdout().flush().unwrap();
    print!("{}{}", cursor::Up(n), clear::AfterCursor);
    io::stdout().flush().unwrap();
}
