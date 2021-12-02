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
                .expect(&format!("Could not parse distance to integer {}", distance));
            (direction, distance)
        })
        .collect()
}

fn solve1(input: &Vec<(&str, i32)>) -> i32 {
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

fn solve2(input: &Vec<(&str, i32)>) -> i32 {
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
