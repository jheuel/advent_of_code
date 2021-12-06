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
    input.lines().map(|line| Line::from(line)).collect()
}

fn count<T>(map: &HashMap<T, u32>) -> u32 {
    map.iter().fold(0, |s, (_, v)| {
        s + match *v > 1 {
            true => 1,
            false => 0,
        }
    })
}

fn solve1(lines: &Vec<Line>) -> u32 {
    let mut field: HashMap<Vector, u32> = HashMap::new();

    for line in lines {
        let d = (line.to - line.from).signum();
        if !(d.horizontal() || d.vertical()) {
            continue;
        }

        let mut p = line.from.clone();
        *field.entry(p).or_insert(0) += 1;
        while p != line.to {
            p = p + d;
            *field.entry(p).or_insert(0) += 1;
        }
    }
    count(&field)
}

fn solve2(lines: &Vec<Line>) -> u32 {
    let mut field: HashMap<Vector, u32> = HashMap::new();

    for line in lines {
        let d = line.to - line.from;
        if !(d.horizontal() || d.vertical() || d.diagonal()) {
            continue;
        }
        let d = d.signum();

        let mut p = line.from.clone();
        *field.entry(p).or_insert(0) += 1;
        while p != line.to {
            p = p + d;
            *field.entry(p).or_insert(0) += 1;
        }
    }
    count(&field)
}
