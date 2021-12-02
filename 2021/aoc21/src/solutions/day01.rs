pub fn solve(input: &str) {
    let input: Vec<i32> = parse(input);

    println!("Part1: {}", solve1(&input));
    println!("Part2: {}", solve2(&input));
}

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| {
            x.parse::<i32>()
                .expect(&format!("failed to parse {} into integer", x))
        })
        .collect()
}

fn window(input: &Vec<i32>, i: usize) -> i32 {
    input[i] + input[i + 1] + input[i + 2]
}

fn solve1(input: &Vec<i32>) -> i32 {
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

fn solve2(input: &Vec<i32>) -> i32 {
    let mut n_increases = 0;
    for i in 0..input.len() - 3 {
        if window(input, i) < window(input, i + 1) {
            n_increases += 1;
        }
    }
    n_increases
}
