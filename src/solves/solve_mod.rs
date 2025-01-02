#[derive(Clone)]
pub struct Solve {
    title: String,
    scenario: Vec<String>,
    input: Vec<String>,
    solve: Option<fn(Vec<String>) -> i32>,
}

impl Solve {
    pub fn from_path(
        dir_path: &str,
        part: u8,
        solve: Option<fn(Vec<String>) -> i32>,
    ) -> Option<Self> {
        use crate::utils;

        let target_file = utils::try_read_file(&format!("{}/scenario{}.txt", dir_path, part));
        let input = utils::try_read_file(&format!("{}/input{}.txt", dir_path, part));

        match (target_file, input.clone()) {
            (a, b) if a.is_none() || b.is_none() => None,
            (a, b) => {
                let title = a.clone().unwrap().into_iter().next().unwrap().clone();
                let title: String = title
                    .split(" ")
                    .filter(|&x| x != "---")
                    .collect::<Vec<_>>()
                    .join(" ")
                    .replace("\n", "");

                let temp_scenario: Vec<String> =
                    a.unwrap().iter().skip(2).map(|x| x.to_string()).collect();

                let mut scenario: Vec<String> = Vec::new();
                let mut line_holder = String::new();

                for row in temp_scenario {
                    for word in row.split(" ") {
                        line_holder.push_str(word);
                        line_holder.push(' ');
                        if line_holder.len() > 100 {
                            scenario.push(line_holder.clone());
                            line_holder.clear();
                        }
                    }

                    scenario.push(line_holder.clone());
                    line_holder.clear();
                }

                if scenario.len() > 40 {
                    scenario = vec![
                        "Scenario is \x1b[1;33;41m too long \x1b[0m to be printed + cleared well :^(".to_string(),
                        "Even so, you can still run the solve...".to_string(),
                        "".to_string(),
                        format!("\x1b[3mYou can fin full content at {dir_path}/input{part}.txt\x1b[0m"),
                    ];
                }
                let input = b.unwrap();

                Some(Self {
                    title,
                    scenario,
                    input,
                    solve,
                })
            }
        }
    }

    pub fn show_data(&self) {
        println!("  \x1b[1;32m{}\x1b[0m\n", self.title);
        println!(
            "{}",
            self.scenario
                .iter()
                .map(|row| format!("  {}", row))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    pub fn get_scenario(&self) -> Vec<String> {
        self.scenario.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn run(&self) {
        if let Some(solve) = self.solve {
            solve(self.input.clone());
        }
    }
}

pub fn get_all_solves() -> Vec<(Option<Solve>, Option<Solve>)> {
    use crate::puzzles::{day01, day02, day03};

    type Function = Option<fn(Vec<String>) -> i32>;
    let fn_vec: Vec<(Function, Function)> = vec![
        (Some(day01::solve1), Some(day01::solve2)),
        (Some(day02::solve1), Some(day02::solve2)),
        (Some(day03::solve1), None),
    ];

    let mut solves_vector: Vec<(Option<Solve>, Option<Solve>)> = Vec::new();

    for i in 1..=25 {
        let (part1, part2): (Function, Function) = if i > fn_vec.len() {
            (None, None)
        } else {
            (fn_vec[i - 1].0, fn_vec[i - 1].1)
        };

        let day_number = match i {
            val if val < 10 => format!("0{}", val),
            val => val.to_string(),
        };

        let (solve1, solve2): (Option<Solve>, Option<Solve>) = (
            Solve::from_path(&format!("./src/puzzles/day{}", day_number), 1, part1),
            Solve::from_path(&format!("./src/puzzles/day{}", day_number), 2, part2),
        );

        solves_vector.push((solve1, solve2));
    }

    solves_vector
}
