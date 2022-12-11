use itertools::Itertools;
use nom::{character::complete::{line_ending, u32},
          combinator::map,
          IResult,
          multi::{count, separated_list0},
};

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = Vec<Vec<usize>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            count(line_ending, 2),
            separated_list0(line_ending, map(u32, |c| c as usize)),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut high_elf_calories = 0;
        for x in input.iter() {
            let total = x.iter().sum();
            if total > high_elf_calories {
                high_elf_calories = total;
            }
        }
        high_elf_calories
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        // method 1
        // let mut totals: Vec<usize> = input.iter().map(|inner| { inner.iter().sum() }).collect::<Vec<usize>>();
        // totals.sort();
        // totals.reverse();
        // *&totals[0..3].iter().sum::<usize>()

        // method 2
        input.iter().map(|inner| { inner.iter().sum::<usize>() }).sorted().rev().take(3).sum()

    }
}
