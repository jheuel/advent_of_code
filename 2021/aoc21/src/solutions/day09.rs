pub fn solve(input: &str) {
    let input = parse(input);

    println!("Part1: {}", solve1(&input));
    println!("Part2: {}", solve2(&input));
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_local_minima(input: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut mins = Vec::new();
    let n_rows = input.len();
    for i in 0..n_rows {
        let n_columns = input[i].len();
        for j in 0..n_columns {
            if i > 0 && input[i - 1][j] <= input[i][j] {
                continue;
            }
            if i < n_rows - 1 && input[i + 1][j] <= input[i][j] {
                continue;
            }
            if j > 0 && input[i][j - 1] <= input[i][j] {
                continue;
            }
            if j < n_columns - 1 && input[i][j + 1] <= input[i][j] {
                continue;
            }
            mins.push((i, j));
        }
    }
    mins
}

fn solve1(input: &[Vec<u8>]) -> u32 {
    let local_minima = find_local_minima(input);
    local_minima
        .iter()
        .map(|(i, j)| input[*i][*j] as u32 + 1)
        .sum()
}

fn grow(input: &[Vec<u8>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut basin = Vec::new();
    let mut untested = vec![(i, j)];

    while let Some((x, y)) = untested.pop() {
        for (m, n) in [(0, 1), (2, 1), (1, 0), (1, 2)] {
            if x + m == 0 || y + n == 0 {
                continue;
            }
            if x + m > input.len() || y + n > input[x].len() {
                continue;
            }

            let new = (x + m - 1, y + n - 1);

            if input[new.0][new.1] == 9 {
                continue;
            }
            if basin.contains(&new) || untested.contains(&new) {
                continue;
            }

            untested.push(new);
        }
        basin.push((x, y));
    }
    basin
}

fn solve2(input: &[Vec<u8>]) -> usize {
    let local_minima = find_local_minima(input);
    let mut basins: Vec<usize> = local_minima
        .iter()
        .map(|(i, j)| grow(input, *i, *j).len())
        .collect();
    basins.sort_by(|a, b| b.cmp(a));
    basins[0..3].iter().product::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "\
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

    #[test]
    fn part1() {
        let input = &parse(TEST_INPUT);
        assert_eq!(solve1(input), 15);
    }

    #[test]
    fn part2() {
        let input = &parse(TEST_INPUT);
        assert_eq!(solve2(input), 1134);
    }
}
