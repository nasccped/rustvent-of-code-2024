pub fn solve1(input: Vec<String>) -> i32 {
    let input: Vec<String> = input
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect();

    let mut count: i32 = 0;

    for row in input.iter() {
        for i in 0..(row.len() - 3) {
            let cur = &row[i..(i + 4)];
            match cur {
                "XMAS" => count += 1,
                "SAMX" => count += 1,
                _ => (),
            }
        }
    }

    for y in 0..(input.len() - 3) {
        for x in 0..(input.get(y).unwrap().len()) {
            let cur = [
                input.get(y).unwrap().chars().nth(x).unwrap(),
                input.get(y + 1).unwrap().chars().nth(x).unwrap(),
                input.get(y + 2).unwrap().chars().nth(x).unwrap(),
                input.get(y + 3).unwrap().chars().nth(x).unwrap(),
            ]
            .iter()
            .map(|char| char.to_string())
            .collect::<Vec<_>>()
            .join("");

            match cur.as_ref() {
                "XMAS" => count += 1,
                "SAMX" => count += 1,
                _ => (),
            }
        }
    }

    for y in 0..(input.len() - 3) {
        for x in 0..(input.get(y).unwrap().len() - 3) {
            let cur = [
                input.get(y).unwrap().chars().nth(x).unwrap(),
                input.get(y + 1).unwrap().chars().nth(x + 1).unwrap(),
                input.get(y + 2).unwrap().chars().nth(x + 2).unwrap(),
                input.get(y + 3).unwrap().chars().nth(x + 3).unwrap(),
            ]
            .iter()
            .map(|char| char.to_string())
            .collect::<Vec<_>>()
            .join("");

            match cur.as_ref() {
                "XMAS" => count += 1,
                "SAMX" => count += 1,
                _ => (),
            }
        }
    }

    let input: Vec<String> = input
        .iter()
        .map(|row| {
            row.chars()
                .rev()
                .map(|char| char.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect();

    for y in 0..(input.len() - 3) {
        for x in 0..(input.get(y).unwrap().len() - 3) {
            let cur = [
                input.get(y).unwrap().chars().nth(x).unwrap(),
                input.get(y + 1).unwrap().chars().nth(x + 1).unwrap(),
                input.get(y + 2).unwrap().chars().nth(x + 2).unwrap(),
                input.get(y + 3).unwrap().chars().nth(x + 3).unwrap(),
            ]
            .iter()
            .map(|char| char.to_string())
            .collect::<Vec<_>>()
            .join("");

            match cur.as_ref() {
                "XMAS" => count += 1,
                "SAMX" => count += 1,
                _ => (),
            }
        }
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
    let mut count: i32 = 0;

    for y in 1..(input.len() - 1) {
        for x in 1..(input[y].len() - 1) {
            let (tl, tr, ct, dl, dr) = (
                input[y - 1].chars().nth(x - 1).unwrap(),
                input[y - 1].chars().nth(x + 1).unwrap(),
                input[y].chars().nth(x).unwrap(),
                input[y + 1].chars().nth(x - 1).unwrap(),
                input[y + 1].chars().nth(x + 1).unwrap(),
            );
            let tl_to_dr = [tl, ct, dr]
                .iter()
                .map(|char| char.to_string())
                .collect::<Vec<_>>()
                .join("");

            let tr_to_dl = [tr, ct, dl]
                .iter()
                .map(|char| char.to_string())
                .collect::<Vec<_>>()
                .join("");

            match (tl_to_dr.as_ref(), tr_to_dl.as_ref()) {
                ("MAS", "MAS") => count += 1,
                ("SAM", "MAS") => count += 1,
                ("MAS", "SAM") => count += 1,
                ("SAM", "SAM") => count += 1,
                _ => (),
            }
        }
    }

    println!("  result: {}", count);
    count
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2};

    #[test]
    fn part1() {
        let sample: Vec<String> = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .iter()
        .map(|row| row.to_string())
        .collect();

        assert_eq!(solve1(sample), 18);
    }

    #[test]
    fn part2() {
        let sample: Vec<String> = [
            ".M.S......",
            "..A..MSMS.",
            ".M.S.MAA..",
            "..A.ASMSM.",
            ".M.S.M....",
            "..........",
            "S.S.S.S.S.",
            ".A.A.A.A..",
            "M.M.M.M.M.",
            "..........",
        ]
        .iter()
        .map(|row| row.to_string())
        .collect();

        assert_eq!(solve2(sample), 9);
    }
}
