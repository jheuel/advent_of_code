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
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let boards: Vec<Board> = input.iter().map(|x| Board::from(x)).collect();

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
