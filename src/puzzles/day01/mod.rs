use crate::visuals as vs;
use vs::get_escape;
use vs::solve_print as svp;

pub fn solve1(input: Vec<String>) {
    let (mut left, mut right) = (
        input
            .iter()
            .map(|x| x.split("   ").next().unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>(),
        input
            .iter()
            .map(|x| x.split("   ").nth(1).unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>(),
    );

    svp(&format!(
        "left: {} elements, right: {} elements",
        left.len(),
        right.len()
    ));
    svp("");

    svp("They both aren't sorted yet...");
    svp(&format!(
        "{}Sorting{}...",
        get_escape(0, 32, 0),
        get_escape(0, 0, 0)
    ));
    svp("");
    svp("");

    left.sort();
    right.sort();

    let mut lval: &i32;
    let mut rval: &i32;

    svp("The first 10 sorted elements:");
    svp("");
    svp(" left  | right ");
    svp(" ------+------");

    for i in 0..10 {
        lval = &left[i];
        rval = &right[i];

        svp(&format!(" {} | {}", lval, rval));
    }

    svp("");
    svp(" ..... | .....");

    let result = left.iter().zip(right.iter()).fold(0, |mut a, (l, r)| {
        a += (l - r).abs();
        a
    });

    svp(&format!(
        "And the result is: {}{}{}",
        get_escape(1, 32, 0),
        result,
        get_escape(0, 0, 0)
    ));
}
