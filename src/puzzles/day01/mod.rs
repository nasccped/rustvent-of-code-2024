use crate::visuals as vs;
use vs::get_escape;
use vs::solve_print as svp;

// part1 solve
pub fn solve1(input: Vec<String>) {
    // necessary mods
    use std::io::{self, Write};
    use std::thread;
    use std::time::Duration;

    // separete input into sides (left | right)
    let (mut left, mut right) = (
        input
            // iterate through input
            .iter()
            // map > foreach row,
            // split from spaces,
            // get the first element (next) and force parse (there only numbers, there's no problem)
            .map(|x| x.split("   ").next().unwrap().parse::<i32>().unwrap())
            // collect they as a vector
            .collect::<Vec<_>>(),
        // same as above, but with the second one <nth(1)>
        input
            .iter()
            .map(|x| x.split("   ").nth(1).unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>(),
    );

    // show elements amoud
    svp(
        &format!(
            "left: {} elements, right: {} elements",
            left.len(),
            right.len()
        ),
        true,
    );
    svp("", true);
    svp("They both aren't sorted yet...", true);
    svp("", true);

    // sorting animation
    svp(
        &format!("{}Sorting{}", get_escape(1, 32, 0), get_escape(0, 0, 0)),
        false,
    );
    io::stdout().flush().unwrap();

    for i in 0..10 {
        // go back dot
        if i > 0 && i % 3 == 0 {
            print!("\x08\x08\x08   \x08\x08\x08");
        }
        // sleep, print dot and flush
        thread::sleep(Duration::from_millis(400));
        print!(".");
        io::stdout().flush().unwrap();
    }
    svp("", true);
    svp("", true);

    // sort both vectors
    left.sort();
    right.sort();

    // prepare ref holder
    let mut lval: &i32;
    let mut rval: &i32;

    // show sorted values => table header:
    svp("The first 10 sorted elements:", true);
    svp("", true);
    svp(" left  | right ", true);
    svp(" ------+------", true);

    // show 10 first elements in both vecs
    for i in 0..10 {
        lval = &left[i];
        rval = &right[i];

        svp(&format!(" {} | {}", lval, rval), true);
    }
    svp(" ..... | .....", true);
    svp("", true);

    // getting resut by zipping left in ring, fold they, start with zero and append diferences
    // between each element in they both
    let result = left.iter().zip(right.iter()).fold(0, |mut a, (l, r)| {
        a += (l - r).abs();
        a
    });

    // print result
    svp(
        &format!(
            "And the result is: {}{}{}",
            get_escape(1, 32, 0),
            result,
            get_escape(0, 0, 0)
        ),
        true,
    );
}

// part 2 solve
pub fn solve2(input: Vec<String>) {
    let left: Vec<i32> = input
        .clone()
        .iter()
        .map(|x| {
            x.split("   ")
                .next()
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0)
        })
        .collect();

    let right: Vec<i32> = input
        .iter()
        .map(|x| {
            x.split("   ")
                .nth(1)
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0)
        })
        .collect();

    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = left.iter().fold(HashMap::<i32, i32>::new(), |mut a, b| {
        a.insert(b.to_owned(), 0);
        a
    });

    for r in right {
        if let Some(pointer) = map.get_mut(&r) {
            *pointer += 1;
        }
    }

    let result_for_each: Vec<i32> = map.iter().map(|x| x.0 * x.1).collect();

    let final_result: i32 = result_for_each.iter().fold(0, |mut a, b| {
        a += b;
        a
    });

    svp(&format!("Final result is: {}", final_result), true)
}
