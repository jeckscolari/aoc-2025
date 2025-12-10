use std::ops::Deref;
use std::str::FromStr;

use crate::days::Solution;

struct Button(u64);

impl Deref for Button {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Target(u64);

impl Deref for Target {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('(').trim_end_matches(')');

        let mut button_bits = 0u64;
        for num_str in s.split(',') {
            if let Ok(idx) = num_str.trim().parse::<usize>() {
                button_bits |= 1 << idx;
            }
        }
        Ok(Button(button_bits))
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut target = 0u64;
        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            }
        }
        Ok(Target(target))
    }
}

struct Machine {
    target: Target,
    buttons: Vec<Button>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bracket_start = s.find('[').ok_or("No [")?;
        let bracket_end = s.find(']').ok_or("No ]")?;
        let pattern = &s[bracket_start + 1..bracket_end];

        let target = pattern.parse::<Target>()?;

        let rest = &s[bracket_end + 1..];
        let brace_start = rest.find('{').ok_or("No {")?;
        let button_section = &rest[..brace_start];

        let buttons: Vec<Button> = button_section
            .split(' ')
            .map(|s| s.parse::<Button>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Machine { target, buttons })
    }
}

impl Machine {
    fn min_presses_for_lights(&self) -> usize {
        let n = self.buttons.len();
        let mut min = usize::MAX;

        for mask in 0..(1u64 << n) {
            let mut state = 0u64;
            let mut presses = 0;

            for (i, button) in self.buttons.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    state ^= **button;
                    presses += 1;
                }
            }

            if state == *self.target {
                min = min.min(presses);
            }
        }

        min
    }
}

pub struct Day10 {
    machines: Vec<Machine>,
}

impl FromStr for Day10 {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let machines = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Day10 { machines })
    }
}

impl Solution for Day10 {
    fn part1(&self) -> String {
        self.machines
            .iter()
            .map(|m| m.min_presses_for_lights())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}
