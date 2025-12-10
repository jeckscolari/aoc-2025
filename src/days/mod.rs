use std::fs;
use std::str::FromStr;
use std::time::{Duration, Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub trait Solution: FromStr {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

type PartResult = (Option<String>, Option<Duration>);
type SolutionResult = (PartResult, PartResult);

pub fn read_input(day: u8) -> Result<String, std::io::Error> {
    let input_path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(input_path)
}

#[macro_export]
macro_rules! time_it {
    ($expr:expr) => {{
        let start = Instant::now();
        let result = $expr;
        let duration = start.elapsed();
        (result, duration)
    }};
}

pub struct DayResult {
    pub day: u8,
    pub input_load_time: Duration,
    pub part1_answer: Option<String>,
    pub part1_time: Option<Duration>,
    pub part2_answer: Option<String>,
    pub part2_time: Option<Duration>,
    pub total_time: Duration,
}

impl DayResult {
    pub fn run(day: u8) -> Option<Self> {
        let (input_result, input_load_time) = time_it!(read_input(day));
        let input = input_result.ok()?;

        let ((part1_answer, part1_time), (part2_answer, part2_time)) = run_solution(day, &input)?;

        let total_time =
            input_load_time + part1_time.unwrap_or_default() + part2_time.unwrap_or_default();

        Some(DayResult {
            day,
            input_load_time,
            part1_answer,
            part1_time,
            part2_answer,
            part2_time,
            total_time,
        })
    }
}

macro_rules! run_day {
    ($input:expr, $day:ty) => {{
        let solution = $input.parse::<$day>().ok()?;

        let (part1_result, part1_time) = time_it!(solution.part1());
        let (part1_answer, part1_time) = if part1_result.is_empty() {
            (None, None)
        } else {
            (Some(part1_result), Some(part1_time))
        };

        let (part2_result, part2_time) = time_it!(solution.part2());
        let (part2_answer, part2_time) = if part2_result.is_empty() {
            (None, None)
        } else {
            (Some(part2_result), Some(part2_time))
        };

        Some(((part1_answer, part1_time), (part2_answer, part2_time)))
    }};
}

fn run_solution(day: u8, input: &str) -> Option<SolutionResult> {
    match day {
        1 => run_day!(input, day01::Day01),
        2 => run_day!(input, day02::Day02),
        3 => run_day!(input, day03::Day03),
        4 => run_day!(input, day04::Day04),
        5 => run_day!(input, day05::Day05),
        6 => run_day!(input, day06::Day06),
        7 => run_day!(input, day07::Day07),
        _ => None,
    }
}
