use std::str;

type Input = (Vec<usize>, Vec<usize>);

pub fn generator(input: &str) -> Input {
    let mut lists: Input = (vec![], vec![]);
    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        lists.0.push(split.next().unwrap().parse::<usize>().unwrap());
        lists.1.push(split.next().unwrap().parse::<usize>().unwrap());
    });
    lists
}

pub fn part1(input: &Input) -> usize {
    let mut lists = input.clone();
    lists.0.sort();
    lists.1.sort();

    let mut diffs: Vec<usize> = vec![];
    for i in 0..lists.0.len() {
        diffs.push(lists.0[i].abs_diff(lists.1[i]));
    }
    diffs.iter().sum()
}

// pub fn part2(input: &Input) -> usize {
//     
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generator(SAMPLE)), 11);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 31);
    // }
}
