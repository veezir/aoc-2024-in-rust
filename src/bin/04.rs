use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Input(Vec<Vec<char>>);

impl Input {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input = Input::default();

        for line in reader.lines() {
            let line = line?;

            let vec = line.chars().collect();
            input.0.push(vec);
        }

        Ok(input)
    }
}

const DAY: &str = "04";
const XMAS: &str = "XMAS";
const XMAS2: &str = "SAMX";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn count_in_sequence(seq: &[char]) -> usize {
    let s = seq.iter().collect::<String>();
    s.matches(XMAS).count() + s.matches(XMAS2).count()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;

        let n = input.0.len();
        let m = input.0[0].len();

        let row_count: usize = input.0.iter().map(|row| count_in_sequence(row)).sum();

        let col_count: usize = (0..m)
            .map(|col| {
                count_in_sequence(&(0..n).map(|row| input.0[row][col]).collect::<Vec<char>>())
            })
            .sum();

        let diag_desc_count: usize = (0..(n + m - 1))
            .map(|start| {
                count_in_sequence(
                    &(0..n)
                        .filter_map(|i| {
                            let j = start as isize - i as isize;
                            if j >= 0 && j < m as isize {
                                Some(input.0[i][j as usize])
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .sum();

        let diag_asc_count: usize = (-(n as isize)..(m as isize))
            .map(|start| {
                count_in_sequence(
                    &(0..n)
                        .filter_map(|i| {
                            let j = i as isize + start;
                            if j >= 0 && j < m as isize {
                                Some(input.0[i][j as usize])
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .sum();

        Ok(row_count + col_count + diag_desc_count + diag_asc_count)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: Input = Input::parse(reader)?;

        let n = input.0.len();
        let res = input
            .0
            .iter()
            .enumerate()
            .filter(|(i, _)| *i >= 1 && *i < n - 1)
            .map(|(i, v)| {
                v.iter()
                    .enumerate()
                    .filter(|(j, c)| **c == 'A' && *j >= 1 && *j < n - 1)
                    .filter_map(|(j, _)| {
                        let no = input.0[i - 1][j - 1];
                        let se = input.0[i + 1][j + 1];
                        let ne = input.0[i - 1][j + 1];
                        let so = input.0[i + 1][j - 1];
                        if (no == 'M' && se == 'S' || no == 'S' && se == 'M')
                            && (ne == 'M' && so == 'S' || ne == 'S' && so == 'M')
                        {
                            return Some(true);
                        }
                        None
                    })
                    .count()
            })
            .sum();

        Ok(res)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
