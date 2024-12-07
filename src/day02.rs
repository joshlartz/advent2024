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
            let asc = report.is_sorted_by(|a, b| a < b);
            let desc = report.is_sorted_by(|a, b| b < a);
            // rule one all increasing or decreasing
            if !(asc || desc) {
                return None;
            }

            report
                .windows(2)
                .all(|pairs| (1..=3).contains(&pairs[0].abs_diff(pairs[1])))
                .then_some(0)
        })
        .count()
}

// pub fn part2(input: &Input) -> usize {

// }

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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 31);
    // }
}
