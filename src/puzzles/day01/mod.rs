pub fn solve1(input: Vec<String>) -> i32 {
    let input: Vec<String> = input
        .iter()
        .filter(|&row| !row.is_empty())
        .map(|x| x.to_string())
        .collect();

    let mut left: Vec<i32> = input
        .iter()
        .map(|pair| pair.split("   ").next().unwrap().parse::<i32>().unwrap())
        .collect();

    let mut right: Vec<i32> = input
        .iter()
        .map(|pair| pair.split("   ").nth(1).unwrap().parse::<i32>().unwrap())
        .collect();

    left.sort();
    right.sort();

    let mut count = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        count += (l - r).abs();
    }

    println!("  result: {}", count);

    count
}

pub fn solve2(input: Vec<String>) -> i32 {
    let input: Vec<String> = input
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect();

    let lefts: Vec<i32> = input
        .iter()
        .map(|row| row.split("   ").next().unwrap().parse::<i32>().unwrap())
        .collect();

    let rights: Vec<i32> = input
        .iter()
        .map(|row| row.split("   ").nth(1).unwrap().parse::<i32>().unwrap())
        .collect();

    let mut similarity_score: i32 = 0;

    for l in lefts.iter() {
        let counter = rights.iter().filter(|&element| element == l).count() as i32;
        similarity_score += counter * (*l);
    }

    println!("  result: {}", similarity_score);
    similarity_score
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2};

    #[test]
    fn part1() {
        let sample: Vec<String> = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
            .iter()
            .map(|row| row.to_string())
            .collect();

        assert_eq!(solve1(sample), 11);
    }

    #[test]
    fn part2() {
        let sample: Vec<String> = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
            .iter()
            .map(|row| row.to_string())
            .collect();

        assert_eq!(solve2(sample), 31);
    }
}
