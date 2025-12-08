use std::str::FromStr;

use crate::days::Solution;

pub struct Day05 {
    ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn length(&self) -> u64 {
        self.end - self.start + 1
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

impl Day05 {
    fn is_fresh(&self, ingredient: u64) -> bool {
        let mut left = 0;
        let mut right = self.ranges.len();

        while left < right {
            let mid = (left + right) / 2;
            let range = &self.ranges[mid];

            if ingredient < range.start {
                right = mid;
            } else if ingredient > range.end {
                left = mid + 1;
            } else {
                return true;
            }
        }

        false
    }
}

impl Solution for Day05 {
    fn from_str(input: &str) -> Self {
        let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
        let mut ranges: Vec<Range> = ranges_str
            .lines()
            .map(|line| line.parse::<Range>().unwrap())
            .collect();
        let ingredients = ingredients_str
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();

        ranges.sort_by_key(|r| r.start);

        let mut merged_ranges: Vec<Range> = Vec::new();
        for range in ranges {
            if merged_ranges.is_empty() || merged_ranges.last().unwrap().end < range.start {
                merged_ranges.push(range);
            } else {
                merged_ranges.last_mut().unwrap().end =
                    merged_ranges.last().unwrap().end.max(range.end);
            }
        }

        Day05 {
            ranges: merged_ranges,
            ingredients,
        }
    }

    fn part1(&self) -> String {
        self.ingredients
            .iter()
            .filter(|&ingredient| self.is_fresh(*ingredient))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.ranges
            .iter()
            .map(|range| range.length())
            .sum::<u64>()
            .to_string()
    }
}
