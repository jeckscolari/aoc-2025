use std::fs;
use std::time::{Duration, Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub trait Solution {
    fn from_str(input: &str) -> Self
    where
        Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

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

        let solution = create_solution(day, &input)?;

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

fn create_solution(day: u8, input: &str) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day01::Day01::from_str(input))),
        2 => Some(Box::new(day02::Day02::from_str(input))),
        3 => Some(Box::new(day03::Day03::from_str(input))),
        4 => Some(Box::new(day04::Day04::from_str(input))),
        5 => Some(Box::new(day05::Day05::from_str(input))),
        _ => None,
    }
}
