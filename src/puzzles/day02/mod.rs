use crate::visuals as vs;
use vs::get_escape;
use vs::solve_print as svp;

enum ReportType {
    Awaiting(i32),
    Increasing(i32),
    Decreasing(i32),
    AlreadyUnsafe,
}

impl ReportType {
    pub fn compare(&self, compare_to: ReportType) -> bool {
        match (self, compare_to) {
            (ReportType::Awaiting(_), ReportType::Awaiting(_)) => true,
            (ReportType::Increasing(_), ReportType::Increasing(_)) => true,
            (ReportType::Decreasing(_), ReportType::Decreasing(_)) => true,
            (ReportType::AlreadyUnsafe, ReportType::AlreadyUnsafe) => true,
            _ => false,
        }
    }
}

pub fn solve1(input: Vec<String>) {
    let mut reports_vec: Vec<ReportType> = Vec::new();

    for report_row in input {
        let current_report: Vec<i32> = report_row.split(" ").fold(Vec::<i32>::new(), |mut a, b| {
            a.push(b.parse().unwrap());
            a
        });

        let report_holder: ReportType =
            current_report
                .iter()
                .fold(ReportType::Awaiting(-1), |a, b| match a {
                    ReportType::AlreadyUnsafe => ReportType::AlreadyUnsafe,
                    ReportType::Awaiting(-1) => ReportType::Awaiting(b.to_owned()),
                    ReportType::Awaiting(x) => {
                        let b_copy = b.to_owned();
                        if !(1..=3).contains(&(x - b).abs()) {
                            ReportType::AlreadyUnsafe
                        } else if x < b_copy {
                            ReportType::Increasing(b_copy)
                        } else {
                            ReportType::Decreasing(b_copy)
                        }
                    }
                    ReportType::Increasing(x) => {
                        let b_copy = b.to_owned();
                        if !(1..=3).contains(&(x - b).abs()) || b < &x {
                            ReportType::AlreadyUnsafe
                        } else {
                            ReportType::Increasing(b_copy)
                        }
                    }
                    ReportType::Decreasing(x) => {
                        let b_copy = b.to_owned();
                        if !(1..=3).contains(&(x - b).abs()) || b > &x {
                            ReportType::AlreadyUnsafe
                        } else {
                            ReportType::Decreasing(b_copy)
                        }
                    }
                });
        reports_vec.push(report_holder);
    }

    reports_vec.retain(|x| !x.compare(ReportType::AlreadyUnsafe));
    let final_result: i32 = reports_vec.len() as i32;

    svp(&format!("The final result is: {}", final_result), true);
}

pub fn solve2(input: Vec<String>) {
    let inputs_as_i32: Vec<Vec<i32>> = input
        .iter()
        .map(|report| {
            report
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    svp(
        &format!(
            "The final result is: {}Still working on it{}",
            get_escape(1, 31, 0),
            get_escape(0, 0, 0)
        ),
        true,
    );
}
