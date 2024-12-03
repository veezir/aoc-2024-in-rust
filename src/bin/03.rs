use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Input(Vec<String>); // TODO

impl Input {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input = Input::default();

        for line in reader.lines() {
            let line = line?;
            input.0.push(line);
        }

        Ok(input)
    }
}

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;
        let mut res = 0;
        let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for string in input.0 {
            res += regex
                .captures_iter(&string)
                .map(|c| c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
                .sum::<usize>();
        }
        Ok(res)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

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
