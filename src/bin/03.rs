use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Input(String); // TODO

impl Input {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input = Input::default();

        for line in reader.lines() {
            let line = line?;
            input.0.push_str(&line);
        }

        Ok(input)
    }
}

const REGEX: &str = r"mul\((\d+),(\d+)\)";

fn compute(slice: &str, regex: &Regex) -> usize {
    regex
        .captures_iter(slice)
        .map(|c| c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
        .sum::<usize>()
}

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;
        let mut res = 0;
        let regex = Regex::new(REGEX).unwrap();
        res += compute(&input.0, &regex);
        Ok(res)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;
        let regex = Regex::new(REGEX).unwrap();
        let res = input
            .0
            .split("do()")
            .map(|s| compute(&s[..s.find("don't()").unwrap_or(s.len())], &regex))
            .sum::<usize>();
        Ok(res)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
