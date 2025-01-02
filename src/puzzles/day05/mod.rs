pub fn solve1(input: Vec<String>) -> i32 {
    use std::collections::HashMap;

    let input: Vec<String> = input.into_iter().filter(|row| !row.is_empty()).collect();
    let break_index = input.iter().position(|brk| brk.starts_with("#")).unwrap();
    let pages_pair: Vec<String> = input[0..break_index]
        .iter()
        .map(|pair| pair.to_string())
        .collect();
    let update_sequence: Vec<String> = input[(break_index + 1)..]
        .iter()
        .map(|seq| seq.to_string())
        .collect();

    let x_pages: HashMap<i32, Vec<i32>> = pages_pair
        .iter()
        .map(|pair| {
            pair.split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(HashMap::<i32, Vec<i32>>::new(), |mut a, b| {
            let x = b.first().unwrap();
            let y = b.get(1).unwrap();
            if let Some(pointer) = a.get_mut(x) {
                pointer.push(y.to_owned());
            } else {
                a.insert(x.to_owned(), vec![y.to_owned()]);
            }

            a
        });

    let y_pages: HashMap<i32, Vec<i32>> = pages_pair
        .iter()
        .map(|pair| {
            pair.split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(HashMap::<i32, Vec<i32>>::new(), |mut a, b| {
            let x = b.first().unwrap();
            let y = b.get(1).unwrap();
            if let Some(pointer) = a.get_mut(y) {
                pointer.push(x.to_owned());
            } else {
                a.insert(y.to_owned(), vec![x.to_owned()]);
            }

            a
        });

    let mut updated_storer: Vec<String> = Vec::new();
    let mut flag: bool;
    for row in update_sequence {
        flag = true;
        let values: Vec<i32> = row
            .split(",")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        for (i, val) in values.iter().enumerate() {
            if !flag {
                break;
            }
            let previous: &[i32] = &values[..i];
            let coming: &[i32] = &values[(i + 1)..];

            flag = previous.iter().fold(true, |mut a, b| {
                if a {
                    a = if let Some(maybe) = x_pages.get(val) {
                        !maybe.contains(b)
                    } else {
                        true
                    };
                }
                a
            });

            flag = coming.iter().fold(flag, |mut a, b| {
                if a {
                    a = if let Some(maybe) = y_pages.get(val) {
                        !maybe.contains(b)
                    } else {
                        true
                    };
                }
                a
            });
        }

        if flag {
            updated_storer.push(row);
        }
    }

    let mut count: i32 = 0;
    let mut mid_i: usize;
    for row in updated_storer {
        mid_i = row.split(",").count() / 2;
        count += row.split(",").nth(mid_i).unwrap().parse::<i32>().unwrap();
    }

    println!("  result: {}", count);
    count
}

pub fn solve2(input: Vec<String>) -> i32 {
    use std::collections::HashMap;

    let input: Vec<String> = input.into_iter().filter(|row| !row.is_empty()).collect();
    let break_index = input.iter().position(|brk| brk.starts_with("#")).unwrap();
    let pages_pair: Vec<String> = input[0..break_index]
        .iter()
        .map(|pair| pair.to_string())
        .collect();
    let update_sequence: Vec<String> = input[(break_index + 1)..]
        .iter()
        .map(|seq| seq.to_string())
        .collect();

    let x_pages: HashMap<i32, Vec<i32>> = pages_pair
        .iter()
        .map(|pair| {
            pair.split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(HashMap::<i32, Vec<i32>>::new(), |mut a, b| {
            let x = b.first().unwrap();
            let y = b.get(1).unwrap();
            if let Some(pointer) = a.get_mut(x) {
                pointer.push(y.to_owned());
            } else {
                a.insert(x.to_owned(), vec![y.to_owned()]);
            }

            a
        });

    let y_pages: HashMap<i32, Vec<i32>> = pages_pair
        .iter()
        .map(|pair| {
            pair.split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(HashMap::<i32, Vec<i32>>::new(), |mut a, b| {
            let x = b.first().unwrap();
            let y = b.get(1).unwrap();
            if let Some(pointer) = a.get_mut(y) {
                pointer.push(x.to_owned());
            } else {
                a.insert(y.to_owned(), vec![x.to_owned()]);
            }

            a
        });

    let mut flag: bool;

    let mut updated_storer: Vec<String> = Vec::new();

    for row in update_sequence {
        flag = true;
        let values: Vec<i32> = row
            .split(",")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        for (i, val) in values.iter().enumerate() {
            if !flag {
                break;
            }
            let previous: &[i32] = &values[..i];
            let coming: &[i32] = &values[(i + 1)..];

            flag = previous.iter().fold(true, |mut a, b| {
                if a {
                    a = if let Some(maybe) = x_pages.get(val) {
                        !maybe.contains(b)
                    } else {
                        true
                    };
                }
                a
            });

            flag = coming.iter().fold(flag, |mut a, b| {
                if a {
                    a = if let Some(maybe) = y_pages.get(val) {
                        !maybe.contains(b)
                    } else {
                        true
                    };
                }
                a
            });
        }

        if !flag {
            updated_storer.push(row);
        }
    }

    let mut nums: Vec<Vec<i32>> = Vec::new();
    for up in updated_storer.iter() {
        let mut vec: Vec<i32> = up
            .split(",")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        let mut ind = 1;
        loop {
            if ind >= vec.len() {
                break;
            }

            let (previous, target) = (vec[ind - 1], vec[ind]);

            if let Some(maybe) = y_pages.get(&previous) {
                if maybe.contains(&target) {
                    vec[ind - 1] = target;
                    vec[ind] = previous;
                    ind = 1;
                    continue;
                }
            }

            if let Some(maybe) = x_pages.get(&target) {
                if maybe.contains(&previous) {
                    vec[ind - 1] = target;
                    vec[ind] = previous;
                    ind = 1;
                    continue;
                }
            }

            ind += 1;
        }
        nums.push(vec);
    }

    let mut count: i32 = 0;
    let mut mid_i: usize;
    for vals in nums {
        mid_i = vals.len() / 2;
        count += vals[mid_i];
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
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "#",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .iter()
        .map(|row| row.to_string())
        .collect();

        assert_eq!(solve1(sample), 143);
    }

    #[test]
    fn part2() {
        let sample: Vec<String> = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "#",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .iter()
        .map(|row| row.to_string())
        .collect();

        assert_eq!(solve2(sample), 123);
    }
}
