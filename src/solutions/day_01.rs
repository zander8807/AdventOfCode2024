use std::collections::{HashMap, HashSet};

use super::Solver;

pub struct DayOneSolver {}

impl Solver<'_> for DayOneSolver {
    fn part_1(&self, input: &'_ [&'_ str]) -> Result<String, ()> {
        let mut col_left: Vec<u64> = Vec::new();
        let mut col_right: Vec<u64> = Vec::new();
        for line in input {
            let mut split = line.split_ascii_whitespace();
            col_left.push(split.next().unwrap().parse().unwrap());
            col_right.push(split.next().unwrap().parse().unwrap());
        }

        col_left.sort();
        col_right.sort();

        let sum: u64 = col_left
            .iter()
            .zip(col_right)
            .map(|(left, right)| right.abs_diff(*left))
            .sum();
        return Ok(sum.to_string());
    }

    fn part_2(&self, input: &'_ [&'_ str]) -> Result<String, ()> {
        let mut left_nums: HashSet<u64> = HashSet::new();
        let mut right_occurrences: HashMap<u64, u64> = HashMap::new();

        for line in input {
            let (left, right): (u64, u64) = {
                let mut split = line.split_ascii_whitespace();
                (
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            };

            left_nums.insert(left);
            *right_occurrences.entry(right).or_default() += 1;
        }

        let sum: u64 = left_nums
            .iter()
            .map(|n| {
                let right_occurrence = right_occurrences.entry(*n).or_default();

                *n * *right_occurrence
            })
            .sum();

        return Ok(sum.to_string());
    }
}
