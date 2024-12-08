type Input = Vec<Vec<usize>>;

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|report| {
            if !rule1(report) {
                return None;
            }

            rule2(report).then_some(0)
        })
        .count()
}

// 317 too low
pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|report| {
            if rule1(report) && rule2(report) {
                return Some(0);
            }

            // needs Problem Dampener correction

            for index in 0..report.len() {
                let mut reduced_report = report.clone();
                reduced_report.remove(index);

                if rule1(&reduced_report) && rule2(&reduced_report) {
                    return Some(0);
                }
            }

            None
        })
        .count()
}

/** all increasing or decreasing */
fn rule1(report: &[usize]) -> bool {
    let asc = report.is_sorted_by(|a, b| a < b);
    let desc = report.is_sorted_by(|a, b| a > b);
    asc || desc
}

/** changing by 1-3 */
fn rule2(report: &[usize]) -> bool {
    report
        .windows(2)
        .all(|levels| (1..=3).contains(&levels[0].abs_diff(levels[1])))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generator(SAMPLE)), 4);
    }
}
