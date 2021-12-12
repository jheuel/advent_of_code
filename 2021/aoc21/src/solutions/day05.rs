use std::collections::HashMap;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn signum(&self) -> Vector {
        Vector {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn vertical(&self) -> bool {
        self.x == 0 && self.y != 0
    }

    fn horizontal(&self) -> bool {
        self.x != 0 && self.y == 0
    }

    fn diagonal(&self) -> bool {
        self.x.abs() == self.y.abs()
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Line {
    from: Vector,
    to: Vector,
}

impl Line {
    fn from(text: &str) -> Line {
        let v: Vec<Vec<i32>> = text
            .split_ascii_whitespace()
            .filter(|&entry| !entry.contains("->"))
            .map(|entry| {
                entry
                    .split(',')
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();

        Line {
            from: Vector {
                x: v[0][0],
                y: v[0][1],
            },
            to: Vector {
                x: v[1][0],
                y: v[1][1],
            },
        }
    }
}

pub fn solve(input: &str) {
    let lines = parse(input);

    println!("Part1: {}", solve1(&lines));
    println!("Part2: {}", solve2(&lines));
}

fn parse(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(|line| Line::from(line))
        .collect()
}

fn count<T>(map: &HashMap<T, u32>) -> u32 {
    map.iter().fold(0, |s, (_, v)| {
        s + match *v > 1 {
            true => 1,
            false => 0,
        }
    })
}

fn solve1(lines: &[Line]) -> u32 {
    let mut field: HashMap<Vector, u32> = HashMap::new();

    for line in lines {
        let d = (line.to - line.from).signum();
        if !(d.horizontal() || d.vertical()) {
            continue;
        }

        let mut p = line.from;
        *field.entry(p).or_insert(0) += 1;
        while p != line.to {
            p = p + d;
            *field.entry(p).or_insert(0) += 1;
        }
    }
    count(&field)
}

fn solve2(lines: &[Line]) -> u32 {
    let mut field: HashMap<Vector, u32> = HashMap::new();

    for line in lines {
        let d = line.to - line.from;
        if !(d.horizontal() || d.vertical() || d.diagonal()) {
            continue;
        }
        let d = d.signum();

        let mut p = line.from;
        *field.entry(p).or_insert(0) += 1;
        while p != line.to {
            p = p + d;
            *field.entry(p).or_insert(0) += 1;
        }
    }
    count(&field)
}

#[cfg(test)]
mod test {
    use super::{parse, solve1, solve2};

    static TEST_INPUT: &str = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

    #[test]
    fn part1() {
        let input = &parse(TEST_INPUT);
        assert_eq!(solve1(input), 5);
    }

    #[test]
    fn part2() {
        let input = &parse(TEST_INPUT);
        assert_eq!(solve2(input), 12);
    }
}
