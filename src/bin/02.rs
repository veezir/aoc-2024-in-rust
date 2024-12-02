use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

struct Input {
    list: Vec<Vec<usize>>,
}

impl Input {
    fn new() -> Input {
        Input { list: Vec::new() }
    }

    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input = Input::new();

        for line in reader.lines() {
            let line = line?;

            // Split line to obtain a record of usizes
            let vec = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            input.list.push(vec);
        }

        Ok(input)
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;
        let mut res = 0;
        for vec in input.list {
            let mut sense: Option<bool> = None;
            let b = vec.windows(2).all(|s| {
                let [i1, i2]: [usize; 2] = s.try_into().unwrap();
                let sense_value = sense.get_or_insert(i1 < i2);
                let diff = if *sense_value {
                    i2 as isize - i1 as isize
                } else {
                    i1 as isize - i2 as isize
                };
                (1..=3).contains(&diff)
            });
            if b {
                res += 1;
            }
        }

        Ok(res)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");

    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     let input: Input = Input::parse(reader)?;
    //     let mut occurrences_by_int = HashMap::new();
    //     for &i in &input.list2 {
    //         *occurrences_by_int.entry(i).or_insert(0) += 1;
    //     }
    //     let res = input
    //         .list1
    //         .iter()
    //         .map(|&i| i * (*occurrences_by_int.get(&i).unwrap_or(&0)))
    //         .sum();
    //     Ok(res)
    // }

    // assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
