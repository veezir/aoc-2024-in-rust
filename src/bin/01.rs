use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

struct Input {
    list1: Vec<usize>,
    list2: Vec<usize>,
}

impl Input {
    fn new() -> Input {
        Input {
            list1: Vec::new(),
            list2: Vec::new(),
        }
    }

    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input = Input::new();

        for line in reader.lines() {
            let line = line?;

            // Split line to obtain 2 usize
            let mut parts = line.split_whitespace();
            if let (Some(s1), Some(s2)) = (parts.next(), parts.next()) {
                // Convert parts in usize and add them to the input
                if let (Ok(i1), Ok(i2)) = (s1.parse(), s2.parse()) {
                    input.list1.push(i1);
                    input.list2.push(i2);
                } else {
                    eprintln!("Conversion error line: {}", line);
                }
            }
        }

        Ok(input)
    }

    fn sort(&mut self) {
        self.list1.sort();
        self.list2.sort();
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut input: Input = Input::parse(reader)?;
        input.sort();
        let res = input
            .list1
            .iter()
            .zip(input.list2.iter())
            .map(|(i1, i2)| i1.abs_diff(*i2))
            .sum();

        Ok(res)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;
        let mut occurrences_by_int = HashMap::new();
        for &i in &input.list2 {
            *occurrences_by_int.entry(i).or_insert(0) += 1;
        }
        let res = input
            .list1
            .iter()
            .map(|&i| i * (*occurrences_by_int.get(&i).unwrap_or(&0)))
            .sum();
        Ok(res)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
