use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    OBSTACLE,
    VISITED,
    FREE,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum Direction {
    #[default]
    TOP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn as_pair(&self) -> (isize, isize) {
        match self {
            Direction::TOP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Input {
    states: Vec<Vec<State>>,
    position: (usize, usize),
    direction: Direction,
    initial_position: (usize, usize),
}

impl Input {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut input = Input::default();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;

            let vec = line
                .chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '^' => {
                        input.position = (i.try_into().unwrap(), j.try_into().unwrap());
                        input.initial_position = (i.try_into().unwrap(), j.try_into().unwrap());
                        State::VISITED
                    }
                    '#' => State::OBSTACLE,
                    _ => State::FREE,
                })
                .collect::<Vec<_>>();
            input.states.push(vec);
        }

        Ok(input)
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::TOP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::TOP,
        }
    }

    fn can_advance(&self) -> bool {
        let n = self.states.len();
        let m = self.states[0].len();
        match (self.position, self.direction.clone()) {
            ((_, 0), Direction::LEFT) => false,
            ((_, val), Direction::RIGHT) if val == m - 1 => false,
            ((0, _), Direction::TOP) => false,
            ((val, _), Direction::DOWN) if val == n - 1 => false,
            _ => true,
        }
    }

    fn is_obstacle_front(&self) -> bool {
        let pair = self.direction.as_pair();
        self.states[(self.position.0 as isize + pair.0) as usize]
            [(self.position.1 as isize + pair.1) as usize]
            == State::OBSTACLE
    }

    fn advance_and_update(&mut self) -> bool {
        let res = self.advance();
        if res {
            self.update_state();
        }
        res
    }

    fn advance(&mut self) -> bool {
        while self.can_advance() && self.is_obstacle_front() {
            self.turn();
        }
        if !self.can_advance() {
            return false;
        }
        let pair = self.direction.as_pair();
        // println!("{:?}, {:?}", self.position, pair);
        self.position = (
            (self.position.0 as isize + pair.0) as usize,
            (self.position.1 as isize + pair.1) as usize,
        );
        true
    }

    fn update_state(&mut self) {
        self.states[self.position.0][self.position.1] = State::VISITED;
    }

    fn is_loop(&mut self) -> bool {
        while self.advance() {
            if self.position == self.initial_position && self.direction == Direction::TOP {
                return true;
            }
        }
        false
    }

    fn add_obstacle(&mut self, i: usize, j: usize) {
        self.states[i][j] = State::OBSTACLE;
    }
    fn remove_obstacle_and_reset(&mut self, i: usize, j: usize) {
        self.states[i][j] = State::FREE;
        self.position = self.initial_position;
        self.direction = Direction::default();
    }
}

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut input: Input = Input::parse(reader)?;
        while input.advance_and_update() {}
        let res = input
            .states
            .iter()
            .map(|v| v.iter().filter(|&s| s == &State::VISITED).count())
            .sum::<usize>();

        Ok(res)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut input: Input = Input::parse(reader)?;
        let n = input.states.len();
        let m = input.states[0].len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if input.states[i][j] == State::FREE && input.initial_position != (i, j) {
                    input.add_obstacle(i, j);
                    //res += input.is_loop() as usize;
                    if input.is_loop() {
                        res += 1;
                        println!("{}{}", i, j);
                    }
                    input.remove_obstacle_and_reset(i, j);
                }
            }
        }
        Ok(res)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
