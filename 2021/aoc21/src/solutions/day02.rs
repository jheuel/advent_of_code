pub fn solve(input: &str) {
    let input = parse(input);

    println!("Part1: {}", solve1(&input));
    println!("Part2: {}", solve2(&input));
}

fn parse(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|x| {
            let mut splits = x.split_ascii_whitespace();
            let direction = splits.next().expect("Could not read direction");
            let distance = splits.next().expect("Could not read distance");
            let distance = distance
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Could not parse distance to integer {}", distance));
            (direction, distance)
        })
        .collect()
}

fn solve1(input: &[(&str, i32)]) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for &(direction, distance) in input {
        match direction {
            "up" => depth -= distance,
            "down" => depth += distance,
            "forward" => horizontal_position += distance,
            unknown => panic!("unknown command: {:?}", unknown),
        }
    }
    horizontal_position * depth
}

fn solve2(input: &[(&str, i32)]) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for &(direction, distance) in input {
        match direction {
            "up" => aim -= distance,
            "down" => aim += distance,
            "forward" => {
                horizontal_position += distance;
                depth += aim * distance;
            }
            unknown => panic!("unknown command: {:?}", unknown),
        }
    }
    horizontal_position * depth
}

#[cfg(test)]
mod test {
    use super::{parse, solve1, solve2};

    static TEST_INPUT: &str = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";

    #[test]
    fn part1() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve1(&input), 150);
    }

    #[test]
    fn part2() {
        let input = parse(TEST_INPUT);
        assert_eq!(solve2(&input), 900);
    }
}
