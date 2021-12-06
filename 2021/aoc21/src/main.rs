use std::env;
mod solutions;
use std::time::Instant;

fn main() {
    let (day, session) = cli();

    let input = aoc21::download_input(&session, day);

    let now = Instant::now();

    solutions::solve(day, &input);

    println!("solution finished in {:?} μs", now.elapsed().as_micros());
}

fn cli() -> (u8, String) {
    let usage = "Usage: aoc21 <day>, where day is an integer between 1 and 24";

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{}", &usage);
        std::process::exit(1);
    }
    let day: u8 = args[1].parse::<u8>().expect(&usage);
    if day < 1 || day > 24 {
        println!("{}", &usage);
        std::process::exit(1);
    }

    let session =
        env::var("AoC").expect("Please set $AoC environment variable containing the session ID");

    (day, session)
}
