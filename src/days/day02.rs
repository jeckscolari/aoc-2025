use std::str::FromStr;

use crate::days::Solution;

pub struct Day02 {
    ranges: Vec<Range>,
}

struct Range {
    start: i64,
    end: i64,
}

trait TwiceRepeating {
    fn repeats_twice(&self) -> bool;
}

impl TwiceRepeating for i64 {
    fn repeats_twice(&self) -> bool {
        let s = self.to_string();
        let len = s.len();
        len.is_multiple_of(2) && s[..len / 2] == s[len / 2..]
    }
}

trait AtLeastTwiceRepeating {
    fn repeats_at_least_twice(&self) -> bool;
}

impl AtLeastTwiceRepeating for i64 {
    fn repeats_at_least_twice(&self) -> bool {
        let s = self.to_string();
        let len = s.len();
        for pattern_len in 1..=len / 2 {
            if len.is_multiple_of(pattern_len) {
                let pattern = &s[..pattern_len];
                let repeated_pattern = pattern.repeat(len / pattern_len);
                if repeated_pattern == s {
                    return true;
                }
            }
        }
        false
    }
}

impl Range {
    fn sum_twice_repeating_ids(&self) -> i64 {
        (self.start..=self.end).filter(|i| i.repeats_twice()).sum()
    }

    fn sum_at_least_twice_repeating_ids(&self) -> i64 {
        (self.start..=self.end)
            .filter(|i| i.repeats_at_least_twice())
            .sum()
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once('-').ok_or("Invalid range")?;
        let start = start_str
            .parse()
            .map_err(|e| format!("Invalid start: {e}"))?;
        let end = end_str.parse().map_err(|e| format!("Invalid end: {e}"))?;
        Ok(Range { start, end })
    }
}

impl Solution for Day02 {
    fn from_str(input: &str) -> Self {
        let ranges = input
            .trim()
            .split(',')
            .map(|range_str| range_str.parse::<Range>().unwrap())
            .collect();

        Day02 { ranges }
    }

    fn part1(&self) -> String {
        self.ranges
            .iter()
            .map(|range| range.sum_twice_repeating_ids())
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.ranges
            .iter()
            .map(|range| range.sum_at_least_twice_repeating_ids())
            .sum::<i64>()
            .to_string()
    }
}
