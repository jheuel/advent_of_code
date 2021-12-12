#[derive(Clone)]
struct Board {
    data: Vec<Vec<(u32, bool)>>,
}

impl Board {
    fn from(text: &str) -> Board {
        Board {
            data: text
                .split('\n')
                .map({
                    |line| {
                        line.split_ascii_whitespace()
                            .map(str::parse::<u32>)
                            .map(Result::unwrap)
                            .map(|entry| (entry, false))
                            .collect()
                    }
                })
                .collect(),
        }
    }

    fn mark(&mut self, val: u32) {
        for line in self.data.iter_mut() {
            for (entry, marked) in line.iter_mut() {
                *marked = *marked || val == *entry;
            }
        }
    }

    fn sum_unmarked(&self) -> u32 {
        self.data.iter().fold(0, |sum_row, line| {
            sum_row
                + line.iter().fold(0, |sum_col, &(value, marked)| {
                    sum_col
                        + match marked {
                            true => 0,
                            false => value,
                        }
                })
        })
    }

    fn is_won(&self) -> bool {
        self.horizontal() || self.vertical()
    }

    fn horizontal(&self) -> bool {
        'outer: for line in self.data.iter() {
            for (_, marked) in line {
                if !marked {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn vertical(&self) -> bool {
        let n_row = self.data.len();
        let n_col = self.data[0].len();
        'outer: for column in 0..n_col {
            for row in 0..n_row {
                if !self.data[row][column].1 {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }
}

pub fn solve(input: &str) {
    let (draw, boards) = parse(input);

    println!("Part1: {}", solve1(&draw, &boards));
    println!("Part2: {}", solve2(&draw, &boards));
}

fn parse(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut input: Vec<&str> = input.trim().split("\n\n").collect();
    let draw: Vec<u32> = input
        .remove(0)
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let boards: Vec<Board> = input.iter().map(|&x| Board::from(x.trim())).collect();

    (draw, boards)
}

fn solve1(draw: &[u32], boards: &[Board]) -> u32 {
    let mut boards: Vec<Board> = boards.to_vec();

    for &number in draw {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.is_won() {
                return number * board.sum_unmarked();
            }
        }
    }
    0
}

fn solve2(draw: &[u32], boards: &[Board]) -> u32 {
    let mut boards: Vec<Board> = boards.to_vec();

    for &number in draw {
        for board in boards.iter_mut() {
            board.mark(number);
        }

        if boards.len() == 1 && boards[0].is_won() {
            return number * boards[0].sum_unmarked();
        }

        boards = boards.into_iter().filter(|board| !board.is_won()).collect();
    }
    0
}

#[cfg(test)]
mod test {
    use super::{parse, solve1, solve2};

    static TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7";

    #[test]
    fn part1() {
        let (draw, boards) = &parse(TEST_INPUT);
        println!("{:?}", boards[0].data);
        println!("{:?}", draw);
        assert_eq!(solve1(&draw, &boards), 4512);
    }

    #[test]
    fn part2() {
        let (draw, boards) = &parse(TEST_INPUT);
        assert_eq!(solve2(&draw, &boards), 1924);
    }
}
