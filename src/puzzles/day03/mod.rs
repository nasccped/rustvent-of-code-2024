pub fn solve1(input: Vec<String>) -> i32 {
    fn is_digit(val: &str) -> bool {
        for c in val.chars() {
            if !c.is_ascii_digit() {
                return false;
            }
        }
        true
    }

    let input: Vec<String> = input
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect();

    let input: Vec<String> = input
        .iter()
        .map(|row| {
            row.split("mul(")
                .skip(1)
                .map(|sub_sp| sub_sp.split(")").next().unwrap_or(""))
                .filter(|sub_sp| {
                    let mut split = sub_sp.split(",");
                    let count = split.clone().count();
                    count == 2 && split.all(|x| !x.is_empty() && is_digit(x))
                })
                .collect::<Vec<_>>()
                .join("|")
        })
        .collect();

    let mut count = 0;

    for row in input.iter() {
        count += row
            .split("|")
            .map(|pair| {
                pair.split(",")
                    .map(|int| int.parse::<i32>().unwrap())
                    .product::<i32>()
            })
            .sum::<i32>();
    }

    println!("  result: {}", count);
    count
}

pub fn solve2(input: Vec<String>) -> i32 {
    fn is_digit(val: &str) -> bool {
        for c in val.chars() {
            if !c.is_ascii_digit() {
                return false;
            }
        }
        true
    }

    let input: Vec<String> = input
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect();

    let input: Vec<String> = input
        .iter()
        .map(|row| {
            row.split("do()")
                .map(|part| part.split("don't()").next().unwrap())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect();

    let input: Vec<String> = input
        .iter()
        .map(|row| {
            row.split("mul(")
                .skip(1)
                .map(|sub_sp| sub_sp.split(")").next().unwrap_or(""))
                .filter(|sub_sp| {
                    let mut split = sub_sp.split(",");
                    let count = split.clone().count();
                    count == 2 && split.all(|x| !x.is_empty() && is_digit(x))
                })
                .collect::<Vec<_>>()
                .join("|")
        })
        .collect();

    let mut count: i32 = 0;

    for row in input {
        count += row
            .split("|")
            .map(|pair| {
                pair.split(",")
                    .map(|int| int.parse::<i32>().unwrap())
                    .product::<i32>()
            })
            .sum::<i32>();
    }

    println!("  result: {}", count);
    count
}

#[cfg(test)]
mod tests {

    use super::{solve1, solve2};

    #[test]
    fn part1() {
        let sample: Vec<String> =
            ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]
                .iter()
                .map(|item| item.to_string())
                .collect();

        assert_eq!(solve1(sample), 161);
    }

    #[test]
    fn part2() {
        let sample: Vec<String> =
            ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]
                .iter()
                .map(|item| item.to_string())
                .collect();

        assert_eq!(solve2(sample), 48);
    }
}
