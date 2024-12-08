use itertools::Itertools;
use regex::Regex;

type Input<'a> = Vec<&'a str>;

pub fn generator(input: &str) -> Input {
    let re = Regex::new(r"(?m)mul\(\d+,\d+\)").unwrap();

    re.find_iter(input).map(|m| m.as_str()).collect_vec()
}

pub fn part1(input: &Input) -> usize {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();

    input
        .iter()
        .map(|each| {
            let caps = re.captures(each).unwrap();
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * caps.get(2).unwrap().as_str().parse::<usize>().unwrap()
        })
        .sum()
}

// pub fn part2(input: &Input) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 161);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 4);
    // }
}
