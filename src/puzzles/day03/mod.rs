use regex::Regex;

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

    let mut count: i32 = 0;
    let reg: Regex = Regex::new(r"mul(\d+,\d+)").unwrap();

    for row in input.iter() {
        // TODO: todo...
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

    let mut count: i32 = 0;

    for row in input {
        let on_filter: Vec<String> = row
            .split("do()")
            .filter(|item| !item.is_empty())
            .map(|row| row.to_string())
            .collect();

        let on_filter: Vec<String> = on_filter
            .iter()
            .map(|item| item.split("don't()").next().unwrap())
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
            .collect();

        let on_filter: Vec<String> = on_filter
            .join("")
            .split("mul(")
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
            .collect();

        let on_filter: Vec<String> = on_filter
            .iter()
            .map(|item| item.split(")").next().unwrap())
            .filter(|pair| pair.split(",").all(is_digit))
            .map(|item| item.to_string())
            .collect();

        println!("> {}", on_filter.join(" | "));

        let on_filter: Vec<i32> = on_filter
            .iter()
            .map(|pair| {
                pair.split(",")
                    .map(|val| val.parse::<i32>().unwrap())
                    .product()
            })
            .collect();

        count += on_filter.iter().sum::<i32>();
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
