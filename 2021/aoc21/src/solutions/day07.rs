pub fn solve(input: &str) {
    let input = parse(input);

    println!("Part1: {}", solve1(&input));
    println!("Part2: {}", solve2(&input));
}

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn solve1(input: &[i32]) -> i32 {
    let median = median(input);
    input.iter().map(|&x| (x - median).abs()).sum()
}

fn median(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    input.sort_unstable();
    input[input.len() / 2]
}

fn mean(input: &[i32]) -> i32 {
    let mean = input.iter().sum::<i32>() as f32 / input.len() as f32;
    mean.ceil() as i32
}

fn solve2(input: &[i32]) -> i32 {
    let mean = mean(input);

    input
        .iter()
        .map(|&x| {
            let distance = (x - mean).abs();
            ((distance * (distance + 1)) / 2).abs()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{parse, solve1, solve2};

    static TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1() {
        let test_result = 37;
        let input = &parse(TEST_INPUT);
        assert_eq!(solve1(input), test_result);
    }

    #[test]
    fn part2() {
        let test_result = 168;
        let input = &parse(TEST_INPUT);
        assert_eq!(solve2(input), test_result);
    }
}
