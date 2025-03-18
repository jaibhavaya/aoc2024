#[derive(Debug)]
struct Report {
    entries: Vec<i32>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        let mut direction = None;
        for window in self.entries.windows(2) {
            let change = window[0] - window[1];

            if change.abs() > 3 || change == 0 {
                return false;
            }

            let curr_direction = if change < 0 { -1 } else { 1 };

            if let Some(prev_direction) = direction {
                if curr_direction != prev_direction {
                    return false;
                }
            } else {
                direction = Some(curr_direction);
            }
        }
        true
    }
}

pub fn solve(input: &str) -> i32 {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|report| report.is_safe())
        .count()
        .try_into()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| -> Report {
            Report {
                entries: line
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
";
        assert_eq!(solve(input), 2);
    }
}
