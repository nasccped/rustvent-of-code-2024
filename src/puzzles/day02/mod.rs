pub fn solve1(input: Vec<String>) -> i32 {
    let input: Vec<String> = input
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect();

    let mut safes: i32 = 0;

    let reports: Vec<Vec<i32>> = input
        .iter()
        .map(|row| {
            row.split(" ")
                .map(|element| element.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    for rp_row in reports {
        let mut cloned = rp_row.clone();

        if cloned.first().unwrap() > cloned.last().unwrap() {
            cloned.reverse();
        }

        let mut is_safe: bool = true;

        for i in 1..cloned.len() {
            if !is_safe {
                break;
            }
            let diff = cloned[i] - cloned[i - 1];
            if !(1..=3).contains(&diff) {
                is_safe = false;
            }
        }
        safes += if is_safe { 1 } else { 0 };
    }

    println!("  result: {}", safes);
    safes
}

pub fn solve2(input: Vec<String>) -> i32 {
    let input: Vec<String> = input
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect();

    let reports: Vec<Vec<i32>> = input
        .iter()
        .map(|row| {
            row.split(" ")
                .map(|element| element.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut safes: i32 = 0;

    for rp_row in reports {
        let mut cloned = rp_row.clone();

        if cloned.first().unwrap() > cloned.last().unwrap() {
            cloned.reverse();
        }

        let mut is_safe = true;
        for i in 1..cloned.len() {
            if !is_safe {
                break;
            }

            let diff = cloned[i] - cloned[i - 1];

            if !(1..=3).contains(&diff) {
                is_safe = false;
            }
        }

        if !is_safe {
            for i in 0..cloned.len() {
                if is_safe {
                    break;
                }
                let mut fixed = cloned.clone();
                fixed.remove(i);
                is_safe = true;
                for j in 1..fixed.len() {
                    if !is_safe {
                        break;
                    }
                    let diff = fixed[j] - fixed[j - 1];
                    if !(1..=3).contains(&diff) {
                        is_safe = false;
                    }
                }
            }
        }

        safes += if is_safe { 1 } else { 0 };
    }
    println!("  result: {}", safes);
    safes
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2};

    #[test]
    fn part1() {
        let sample: Vec<String> = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .iter()
        .map(|row| row.to_string())
        .collect();

        assert_eq!(solve1(sample), 2);
    }

    #[test]
    fn part2() {
        let sample: Vec<String> = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .iter()
        .map(|row| row.to_string())
        .collect();

        assert_eq!(solve2(sample), 4);
    }
}
