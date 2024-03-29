pub fn solve(input: &str) {
    let input = parse(input);

    println!("Part1: {}", simulate(&input, 80));
    println!("Part2: {}", simulate(&input, 256));
}

fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect()
}

fn simulate(fishes: &[u8], days: u16) -> u64 {
    const N_GENERATIONS: usize = 10;

    let mut generation = [0; N_GENERATIONS + 1];
    // layout for input: "3,4,3,1,2":
    // ready to rumble | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
    //        0        | 1 | 1 | 2 | 1 | 0 | 0 | 0 | 0 | 0 |  0 |
    // overwrite from left to right, use 10 to reset 9

    for &f in fishes {
        generation[f as usize] += 1;
    }

    for _ in 0..days {
        for i in 0..N_GENERATIONS {
            generation[i] = generation[i + 1];
        }

        generation[7] += generation[0];
        generation[9] += generation[0];
    }
    generation[1..9].iter().sum()
}

#[cfg(test)]
mod test {
    use super::{parse, simulate};

    static TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part1() {
        let input = &parse(TEST_INPUT);
        assert_eq!(simulate(input, 18), 26);
        assert_eq!(simulate(input, 80), 5934);
    }

    #[test]
    fn part2() {
        let input = &parse(TEST_INPUT);
        assert_eq!(simulate(input, 256), 26984457539);
    }
}
