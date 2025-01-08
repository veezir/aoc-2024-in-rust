use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// A key represents a stone.
// A value represents the number of this stone in the set
#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Input(HashMap<usize, usize>);

impl Input {
    fn parse<R: BufRead>(mut reader: R) -> Result<Self> {
        let mut input = Input::default();

        let mut line = String::new();
        reader.read_to_string(&mut line).unwrap();
        input.0 = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .fold(HashMap::new(), |mut acc, num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            });

        Ok(input)
    }

    fn blink_one_stone(key: usize, value: usize) -> HashMap<usize, usize> {
        if key == 0 {
            return HashMap::from([(1, value)]);
        }
        let digits = ((key as f64).log10().floor() as usize) + 1;
        if digits % 2 == 0 {
            let half_digits: u32 = (digits / 2) as u32;
            let div = 10_usize.pow(half_digits);
            let mut map = HashMap::new();
            map.insert(key / div, value);
            *map.entry(key % div).or_insert(0) += value;
            map
        } else {
            HashMap::from([(2024 * key, value)])
        }
    }

    fn blink(&mut self) {
        self.0 = self
            .0
            .iter()
            .flat_map(|(&key, &value)| Self::blink_one_stone(key, value))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                *acc.entry(key).or_insert(0) += value;
                acc
            });
    }

    fn blink_several_times(&mut self, n: usize) {
        for _i in (0..n) {
            //println!("Generation {} : {:?}", _i + 1, self.0);
            self.blink();
        }
    }
}

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, n: usize) -> Result<usize> {
        let mut input: Input = Input::parse(reader)?;
        input.blink_several_times(n);
        Ok(input.0.iter().map(|(_, v)| v).sum())
    }

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 6)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 25)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 6)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 75)?);
    println!("Result = {}", result);

    Ok(())
}
