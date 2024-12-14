// will hold every Challenge value as options (better to handle)
pub struct Challenge {
    title: Option<String>,
    context: Option<String>,
    input: Option<Vec<String>>,
}

// Challenge funcs
impl Challenge {
    // create a new one by getting both paths (challenge/input)
    pub fn new(challenge_path: String, input_path: String) -> Self {
        // necessary mods/libs
        use std::fs::File;
        use std::io::Read;

        // holder string to read files + struct values
        let mut read_holder: String = String::new();
        let mut ttl: Option<String> = None;
        let mut ctxt: Option<String> = None;
        let mut inpt: Option<Vec<String>> = None;

        // if challenge file opening results in no error
        if let Ok(mut x) = File::open(challenge_path) {
            // put content into holder string
            let _ = x.read_to_string(&mut read_holder);

            // get first line and remove the '---' things + insert it into title var
            ttl = read_holder
                .lines()
                .nth(0)
                .map(|line| line.to_string().replace("---", ""));

            // get all lines (starting from second) + join each one by new line + put into context
            // var
            ctxt = if read_holder.lines().count() > 2 {
                Some(read_holder.lines().skip(2).collect::<Vec<_>>().join("\n"))
            } else {
                None
            };
        }

        // clear holder
        read_holder.clear();

        // if input file opening results in no error
        if let Ok(mut x) = File::open(input_path) {
            // put values into holder
            let _ = x.read_to_string(&mut read_holder);
            // if holder is not empty, get each line, turn each one into string and collect they
            // into a Vec<String>
            inpt = if !read_holder.is_empty() {
                Some(read_holder.lines().map(|x| x.to_string()).collect())
            } else {
                None
            };
        }

        // return struct with its values (Option<_>)
        Challenge {
            title: ttl,
            context: ctxt,
            input: inpt,
        }
    }
}

// will hold every Solve value => challenge: mandatory + solve_function: optional
pub struct Solve {
    to_solve: Challenge,
    solve: Option<fn(Vec<String>)>,
}

// Solve funcs
impl Solve {
    // create a new one by getting his values
    pub fn new(problem: Challenge, solution: Option<fn(Vec<String>)>) -> Self {
        Solve {
            to_solve: problem,
            solve: solution,
        }
    }

    // getting his challenge by destructuring it
    pub fn get_challenge_values(&self) -> (Option<String>, Option<String>, Option<Vec<String>>) {
        (
            self.to_solve.title.clone(),
            self.to_solve.context.clone(),
            self.to_solve.input.clone(),
        )
    }

    // run `to_solve: Option<fn(Vec<String>)>` function
    pub fn run_solve(&self, input: Vec<String>) {
        // solving directly by `self.solve(...)` doesn't work, so... we need to clone and call it
        let function: fn(Vec<String>) = self.solve.unwrap();
        function(input);
    }
}

// will hold every AdventDay value => day number + solve 1 and 2 (each one is optional)
pub struct AdventDay {
    day_number: u8,
    solves: (Option<Solve>, Option<Solve>),
}

// AdventDay funcs
impl AdventDay {
    // create a new one (same as above)
    pub fn new(day: u8, solutions: (Option<Solve>, Option<Solve>)) -> Self {
        AdventDay {
            day_number: day,
            solves: solutions,
        }
    }

    // simple getter to day
    pub fn get_day_number(&self) -> u8 {
        self.day_number
    }

    // simple getter to (solve is here!)
    pub fn solves_as_bool(&self) -> (bool, bool) {
        (self.solves.0.is_some(), self.solves.1.is_some())
    }

    // return optional solve reference
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

// importing our solve mods
mod day01;
mod day02;

// create a vector with each AdventDay
pub fn gen_advent_days() -> Vec<AdventDay> {
    // type alias
    type OptionalSolve = Option<fn(Vec<String>)>;

    // returnable
    let mut advent_collection: Vec<AdventDay> = Vec::new();

    // preparing already solve functions
    let solutions: Vec<(OptionalSolve, OptionalSolve)> = vec![
        (Some(day01::solve1), Some(day01::solve2)),
        (Some(day02::solve1), None),
    ];

    // iterate from day 1 to day 25
    for i in 1..=25 {
        // creating challenge (part 1) by file path's
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

        // creating challenge (part 2) by file path's
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

        // also looks kinda noob, there's no an easy way to match these guys in less than 20 lines
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

        // create advent with previous collected values
        let current_advent = AdventDay::new(i as u8, (part1, part2));

        // push tey to our collection
        advent_collection.push(current_advent);
    }
    advent_collection
}
