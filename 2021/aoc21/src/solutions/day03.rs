pub fn solve(input: &str) {
    let (n, input) = parse(input);

    println!("Part1: {}", solve1(n, &input));
    println!("Part2: {}", solve2(n, &input));
}

fn parse(input: &str) -> (usize, Vec<i32>) {
    let n = input
        .lines()
        .next()
        .expect("Expect at least one line of input")
        .trim()
        .len();
    let input = input
        .lines()
        .map(str::trim)
        .map(|x| {
            i32::from_str_radix(x, 2).unwrap_or_else(|_| panic!("Could not parse number: {}", x))
        })
        .collect();
    (n, input)
}

fn solve1(n: usize, input: &[i32]) -> i32 {
    let (ones, zeroes) = count(n, input);

    let mut gamma = 0;

    for bit in (0..n).rev() {
        if ones[bit] > zeroes[bit] {
            gamma += 1 << bit;
        }
    }
    let epsilon = !gamma & ((1 << n) - 1);
    gamma * epsilon
}

fn count(n: usize, input: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut ones = vec![0; n];
    let mut zeroes = vec![0; n];

    for inp in input {
        for bit in (0..n).rev() {
            if (inp & (1 << (bit))) > 0 {
                ones[bit] += 1;
            } else {
                zeroes[bit] += 1;
            }
        }
    }
    (ones, zeroes)
}

fn solve2(n: usize, input: &[i32]) -> i32 {
    let oxygen = find_rating(n, input, |x, y| x >= y);
    let carbon = find_rating(n, input, |x, y| x < y);

    oxygen * carbon
}

fn find_rating(n: usize, input: &[i32], compare_fn: fn(i32, i32) -> bool) -> i32 {
    let mut input = input.to_vec();

    for bit in (0..n).rev() {
        let (ones, zeroes) = count(n, &input);
        let most_common = compare_fn(ones[bit], zeroes[bit]) as i32;

        input = input
            .iter()
            .filter(|&x| (x & (1 << bit)) == (most_common << bit))
            .copied()
            .collect();

        if input.len() < 2 {
            break;
        }
    }
    input[0]
}

#[cfg(test)]
mod test {
    use super::{parse, solve1, solve2};

    static TEST_INPUT: &str = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

    #[test]
    fn part1() {
        let (n, input) = parse(TEST_INPUT);
        assert_eq!(solve1(n, &input), 198);
    }

    #[test]
    fn part2() {
        let (n, input) = parse(TEST_INPUT);
        assert_eq!(solve2(n, &input), 230);
    }
}
