mod day01;
mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;

pub fn solve(day: u8, input: &str) {
    match day {
        01 => day01::solve(input),
        02 => day02::solve(input),
        // 03 => day03::solve(input),
        // 04 => day04::solve(input),
        // 05 => day05::solve(input),
        // 06 => day06::solve(input),
        // 07 => day07::solve(input),
        // 08 => day08::solve(input),
        // 09 => day09::solve(input),
        // 10 => day10::solve(input),
        // 11 => day11::solve(input),
        // 12 => day12::solve(input),
        // 13 => day13::solve(input),
        // 14 => day14::solve(input),
        // 15 => day15::solve(input),
        // 16 => day16::solve(input),
        // 17 => day17::solve(input),
        // 18 => day18::solve(input),
        // 19 => day19::solve(input),
        // 20 => day20::solve(input),
        // 21 => day21::solve(input),
        // 22 => day22::solve(input),
        // 23 => day23::solve(input),
        // 24 => day24::solve(input),
        _ => {
            println!("not implemented yet");
            std::process::exit(1);
        }
    }
}
