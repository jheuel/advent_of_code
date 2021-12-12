pub fn solve(input: &str) {
    let input: Vec<i32> = parse(input);

    println!("Part1: {}", solve1(&input));
    println!("Part2: {}", solve2(&input));
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(str::trim)
        .map(|x| {
            x.parse::<i32>()
                .unwrap_or_else(|_| panic!("failed to parse {} into integer", x))
        })
        .collect()
}

fn window(input: &[i32], i: usize) -> i32 {
    input[i] + input[i + 1] + input[i + 2]
}

fn solve1(input: &[i32]) -> i32 {
    let mut last = input[0];
    let mut n_increases = 0;
    for &current in input {
        if last < current {
            n_increases += 1;
        }
        last = current;
    }
    n_increases
}

fn solve2(input: &[i32]) -> i32 {
    let mut n_increases = 0;
    for i in 0..input.len() - 3 {
        if window(input, i) < window(input, i + 1) {
            n_increases += 1;
        }
    }
    n_increases
}

#[cfg(test)]
mod test {
    use super::{parse, solve1, solve2};

    static TEST_INPUT: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    #[test]
    fn part1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve1(&input), 7);
    }

    #[test]
    fn part2() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve2(&input), 5);
    }
}
