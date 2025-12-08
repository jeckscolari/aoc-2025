use std::str::FromStr;

use crate::days::Solution;

pub struct Day03 {
    banks: Vec<Bank>,
}

struct Bank {
    batteries: Vec<u64>,
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(|b| {
                b.to_digit(10)
                    .map(|d| d as u64)
                    .ok_or_else(|| format!("Invalid digit: {b}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Bank { batteries })
    }
}

impl Bank {
    fn max_joltage(&self, k: usize) -> u64 {
        let mut stack: Vec<u64> = Vec::with_capacity(k);

        for (i, &digit) in self.batteries.iter().enumerate() {
            // Greedly remove digits from the stack if they are less than the current digit and we have enough remaining digits to fill k positions
            while !stack.is_empty()
                && stack.last().unwrap() < &digit
                && stack.len() + (self.batteries.len() - i) > k
            {
                stack.pop();
            }

            if stack.len() < k {
                stack.push(digit);
            }
        }

        stack.iter().fold(0u64, |acc, &digit| acc * 10 + digit)
    }
}

impl Solution for Day03 {
    fn from_str(input: &str) -> Self {
        let banks = input
            .lines()
            .map(|line| line.parse::<Bank>().unwrap())
            .collect();
        Day03 { banks }
    }

    fn part1(&self) -> String {
        self.banks
            .iter()
            .map(|bank| bank.max_joltage(2))
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.banks
            .iter()
            .map(|bank| bank.max_joltage(12))
            .sum::<u64>()
            .to_string()
    }
}
