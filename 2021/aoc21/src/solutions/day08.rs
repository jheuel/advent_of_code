pub fn solve(input: &str) {
    let (left, right) = parse(input);

    println!("Part1: {}", solve1(&right));
    println!("Part2: {}", solve2(&left, &right));
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input.trim().lines().for_each(|line| {
        let mut splits = line.trim().split('|');

        let l = splits.next().unwrap();
        let r = splits.next().unwrap();

        let conv = |x: &str| {
            x.trim()
                .split_ascii_whitespace()
                .map(|x| x.chars().map(|c| 1 << (c as u32 - 'a' as u32)).sum::<u8>())
                .collect()
        };

        left.push(conv(l));
        right.push(conv(r));
    });
    (left, right)
}

fn solve1(right: &[Vec<u8>]) -> u32 {
    let known = [2, 3, 4, 7];
    right
        .iter()
        .map(|x| {
            x.iter()
                .map(|&y| known.contains(&y.count_ones()) as u32)
                .sum::<u32>()
        })
        .sum()
}

fn find_remove<F>(input: &mut Vec<u8>, requirement: F) -> u8
where
    F: Fn(&u8) -> bool,
{
    let res = *input.iter().find(|&x| requirement(x)).unwrap();

    // filter in-place
    input.retain(|&x| !requirement(&x));

    res
}

fn find_mapping(line: &[u8]) -> [u8; 10] {
    let mut numbers = [0u8; 10];
    let mut inp = line.to_vec();

    numbers[1] = find_remove(&mut inp, |&x| x.count_ones() == 2);
    numbers[7] = find_remove(&mut inp, |&x| x.count_ones() == 3);
    numbers[4] = find_remove(&mut inp, |&x| x.count_ones() == 4);
    numbers[8] = find_remove(&mut inp, |&x| x.count_ones() == 7);

    // 5 segments
    numbers[2] = find_remove(&mut inp, |x| {
        x.count_ones() == 5 && (*x & numbers[4]).count_ones() == 2
    });
    numbers[3] = find_remove(&mut inp, |x| {
        x.count_ones() == 5 && (*x & numbers[7]).count_ones() == 3
    });
    numbers[5] = find_remove(&mut inp, |x| x.count_ones() == 5);

    // 6 segments
    numbers[9] = find_remove(&mut inp, |x| *x == (numbers[5] | numbers[7]));
    numbers[6] = find_remove(&mut inp, |x| {
        x.count_ones() == 6 && numbers[8] == (*x | numbers[1])
    });
    numbers[0] = find_remove(&mut inp, |x| x.count_ones() == 6);

    numbers
}

fn solve2(left: &[Vec<u8>], right: &[Vec<u8>]) -> u32 {
    left.iter()
        .zip(right)
        .map(|(l, r)| {
            r.iter()
                .rev()
                .enumerate()
                .map(|(i, x)| -> u32 {
                    find_mapping(l).iter().position(|y| y == x).unwrap() as u32
                        * u32::pow(10, i.try_into().unwrap())
                })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{parse, solve1, solve2};

    static TEST_INPUT: &str = "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1() {
        let (_, right) = &parse(TEST_INPUT);
        assert_eq!(solve1(right), 26);
    }

    #[test]
    fn part2() {
        let (left, right) = &parse(TEST_INPUT);
        assert_eq!(solve2(left, right), 61229);
    }
}
