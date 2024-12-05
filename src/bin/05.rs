use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Input {
    rules: BTreeSet<(usize, usize)>,
    records: Vec<Vec<usize>>,
}

impl Input {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input = Input::default();

        let mut records = false;
        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                records = true;
            } else if !records {
                input.rules.insert(
                    line.split('|')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
            } else {
                input.records.push(
                    line.split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect(),
                );
            }
        }

        Ok(input)
    }
}

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;
        let res = input
            .records
            .iter()
            .filter_map(|v| {
                if v.windows(2).all(|p| !input.rules.contains(&(p[1], p[0]))) {
                    Some(v[v.len() / 2])
                } else {
                    None
                }
            })
            .sum::<usize>();

        Ok(res)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //    let input: Input = Input::parse(reader)?;
    //    let mut res = 0;
    //    // TODO
    //    Ok(res)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
