use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Trend {
    Unknown,
    Decreasing,
    Increasing,
}

fn parse_input(input: &'static str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &[i32]) -> bool {
    let mut trend = Trend::Unknown;
    for (n1, n2) in report.iter().tuple_windows() {
        match n1 - n2 {
            0 => return false,
            1..=3 => {
                trend = match trend {
                    Trend::Unknown => Trend::Increasing,
                    Trend::Decreasing => return false,
                    Trend::Increasing => Trend::Increasing,
                }
            }
            -3..=-1 => {
                trend = match trend {
                    Trend::Unknown => Trend::Decreasing,
                    Trend::Decreasing => Trend::Decreasing,
                    Trend::Increasing => return false,
                }
            }
            _ => return false,
        }
    }

    true
}

fn safe_reports_count(input: &'static str) -> usize {
    let reports = parse_input(input);
    reports.into_iter().filter(|r| is_report_safe(r)).count()
}

fn part_1(input: &'static str) {
    let count = safe_reports_count(input);
    println!("Safe reports count is {count}")
}

fn is_safe_lossy(report: &[i32]) -> bool {
    if is_report_safe(report) {
        return true;
    }

    for idx in 0..report.len() {
        let mut r = report.to_vec();
        r.remove(idx);
        if is_report_safe(&r) {
            return true;
        }
    }

    false
}

fn lossy_report_count(input: &'static str) -> usize {
    let reports = parse_input(input);
    reports.into_iter().filter(|r| is_safe_lossy(r)).count()
}

fn part_2(input: &'static str) {
    let lossy = lossy_report_count(input);
    println!("Lossy reports count is {lossy}")
}

pub fn run() {
    let input = crate::utls::read_text_from_file("24", "02").leak();
    part_1(input);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_solution() {
        let counts = safe_reports_count(INPUT);
        assert_eq!(counts, 2);

        let lossy = lossy_report_count(INPUT);
        assert_eq!(lossy, 4);
    }
}
