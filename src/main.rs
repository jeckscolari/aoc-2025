mod days;
mod matrix;
mod report;

use report::Report;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Running all available days...\n");
            let report = Report::from_day_range(1..=25);
            println!("{}", report);
        }
        2 => match args[1].parse::<u8>() {
            Ok(day) if (1..=25).contains(&day) => {
                println!("Running day {}...\n", day);
                let report = Report::from_day_range(day..=day);
                if report.results.is_empty() {
                    eprintln!("Error: No solution found for day {}", day);
                    std::process::exit(1);
                }
                println!("{}", report);
            }
            Ok(day) => {
                eprintln!("Error: Day must be between 1 and 25, got {}", day);
                std::process::exit(1);
            }
            Err(_) => {
                eprintln!("Error: Invalid day number '{}'", args[1]);
                eprintln!("Usage: {} [day]", args[0]);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Usage: {} [day]", args[0]);
            eprintln!("  day: Optional day number (1-25). If omitted, runs all days.");
            std::process::exit(1);
        }
    }
}
