pub struct Challenge {
    title: Option<String>,
    context: Option<String>,
    input: Option<Vec<String>>,
}

impl Challenge {
    pub fn new(challenge_path: String, input_path: String) -> Self {
        use std::fs::File;
        use std::io::Read;

        let mut read_holder: String = String::new();
        let mut ttl: Option<String> = None;
        let mut ctxt: Option<String> = None;
        let mut inpt: Option<Vec<String>> = None;

        if let Ok(mut x) = File::open(challenge_path) {
            let _ = x.read_to_string(&mut read_holder);
            ttl = read_holder
                .lines()
                .nth(0)
                .map(|line| line.to_string().replace("---", ""));
            ctxt = if read_holder.lines().count() > 2 {
                Some(read_holder.lines().skip(2).collect::<Vec<_>>().join("\n"))
            } else {
                None
            };
        }

        read_holder.clear();

        if let Ok(mut x) = File::open(input_path) {
            let _ = x.read_to_string(&mut read_holder);
            inpt = if !read_holder.is_empty() {
                Some(read_holder.lines().map(|x| x.to_string()).collect())
            } else {
                None
            };
        }

        Challenge {
            title: ttl,
            context: ctxt,
            input: inpt,
        }
    }
}

pub struct Solve {
    to_solve: Challenge,
    solve: Option<fn(Vec<String>)>,
}

impl Solve {
    pub fn new(problem: Challenge, solution: Option<fn(Vec<String>)>) -> Self {
        Solve {
            to_solve: problem,
            solve: solution,
        }
    }

    pub fn get_challenge_values(&self) -> (Option<String>, Option<String>, Option<Vec<String>>) {
        (
            self.to_solve.title.clone(),
            self.to_solve.context.clone(),
            self.to_solve.input.clone(),
        )
    }

    pub fn run_solve(&self, input: Vec<String>) {
        let function: fn(Vec<String>) = self.solve.unwrap();
        function(input);
    }
}

pub struct AdventDay {
    day_number: u8,
    solves: (Option<Solve>, Option<Solve>),
}

impl AdventDay {
    pub fn new(day: u8, solutions: (Option<Solve>, Option<Solve>)) -> Self {
        AdventDay {
            day_number: day,
            solves: solutions,
        }
    }

    pub fn get_day_number(&self) -> u8 {
        self.day_number
    }

    pub fn solves_as_bool(&self) -> (bool, bool) {
        (self.solves.0.is_some(), self.solves.1.is_some())
    }

    pub fn get_solve(&self, side: i32) -> Option<&Solve> {
        // trust me. There's no an simple way to do this thing works.
        // looks kinda noob, i know
        if side == 0 {
            self.solves.0.as_ref()
        } else if side == 1 {
            self.solves.1.as_ref()
        } else {
            None
        }
    }
}

mod day01;

pub fn gen_advent_days() -> Vec<AdventDay> {
    type OptionalSolve = Option<fn(Vec<String>)>;
    let mut result: Vec<AdventDay> = Vec::new();

    let solutions: Vec<(OptionalSolve, OptionalSolve)> = vec![(Some(day01::solve1), None)];

    for i in 1..=25 {
        let chall1 = Challenge::new(
            format!(
                "src/puzzles/day{}/chall1.txt",
                if i < 10 {
                    format!("0{}", i)
                } else {
                    i.to_string()
                }
            ),
            format!(
                "src/puzzles/day{}/input1.txt",
                if i < 10 {
                    format!("0{}", i)
                } else {
                    i.to_string()
                }
            ),
        );

        let chall2 = Challenge::new(
            format!(
                "src/puzzles/day{}/chall2.txt",
                if i < 10 {
                    format!("0{}", i)
                } else {
                    i.to_string()
                }
            ),
            format!(
                "src/puzzles/day{}/input2.txt",
                if i < 10 {
                    format!("0{}", i)
                } else {
                    i.to_string()
                }
            ),
        );

        let (part1, part2) = if i <= solutions.len() {
            (
                match solutions[i - 1].0 {
                    x if x.is_some() => Some(Solve::new(chall1, x)),
                    _ => None,
                },
                match solutions[i - 1].1 {
                    x if x.is_some() => Some(Solve::new(chall2, x)),
                    _ => None,
                },
            )
        } else {
            (None, None)
        };

        let current_advent = AdventDay::new(i as u8, (part1, part2));
        result.push(current_advent);
    }
    result
}
