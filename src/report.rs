use std::fmt::Display;
use std::ops::RangeInclusive;
use std::time::Duration;
use comfy_table::Table;
use crate::days::DayResult;

pub struct Report {
    pub results: Vec<DayResult>,
}

impl Report {
    pub fn from_day_range(days: RangeInclusive<u8>) -> Self {
        let results = days.filter_map(DayResult::run).collect();
        Self { results }
    }

    pub fn total_time(&self) -> Duration {
        self.results.iter().map(|r| r.total_time).sum()
    }

    fn format_duration(duration: Duration) -> String {
        let micros = duration.as_micros();
        if micros < 1_000 {
            format!("{} µs", micros)
        } else if micros < 1_000_000 {
            format!("{:.2} ms", micros as f64 / 1_000.0)
        } else {
            format!("{:.2} s", micros as f64 / 1_000_000.0)
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.set_header(vec![
            "Day",
            "Load Time",
            "Part 1 Answer",
            "Part 1 Time",
            "Part 2 Answer",
            "Part 2 Time",
            "Total",
        ]);

        for result in &self.results {
            table.add_row(vec![
                format!("{:02}", result.day),
                Self::format_duration(result.input_load_time),
                result.part1_answer.as_deref().unwrap_or("-").to_string(),
                result.part1_time.map(Self::format_duration).unwrap_or("-".to_string()),
                result.part2_answer.as_deref().unwrap_or("-").to_string(),
                result.part2_time.map(Self::format_duration).unwrap_or("-".to_string()),
                Self::format_duration(result.total_time),
            ]);
        }

        writeln!(f, "{}", table)?;
        writeln!(f, "\nTotal time: {}", Self::format_duration(self.total_time()))
    }
}
