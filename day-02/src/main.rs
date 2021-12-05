use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct SubState {
    x: i32,
    y: i32,
    aim: i32,
}

impl SubState {
    fn new(x: i32, y: i32, aim: i32) -> Self {
        Self { x, y, aim }
    }

    fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    fn product(self) -> i32 {
        self.x * self.y
    }
}

#[derive(Debug, PartialEq)]
enum Movement {
    Forward(i32),
    Aim(i32),
}

impl Add<Movement> for SubState {
    type Output = SubState;

    fn add(self, rhs: Movement) -> Self::Output {
        match rhs {
            Movement::Forward(distance) => {
                Self::new(self.x + distance, self.y + self.aim * distance, self.aim)
            }
            Movement::Aim(aim_change) => Self::new(self.x, self.y, self.aim + aim_change),
        }
    }
}

impl Sum<Movement> for SubState {
    fn sum<I: Iterator<Item = Movement>>(iter: I) -> Self {
        iter.fold(SubState::zero(), |accum, it| accum + it)
    }
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(' ');
        let (direction, amount_str) = split.expect("Failed to split input string");
        let amount: i32 = amount_str.parse().expect("Failed to parse amount");
        match direction {
            "forward" => Ok(Movement::Forward(amount)),
            "up" => Ok(Movement::Aim(-amount)),
            "down" => Ok(Movement::Aim(amount)),
            _ => Err(()),
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let sub_state: SubState = lines
            .flatten()
            .map(|instruction| Movement::from_str(instruction.as_str()))
            .flatten()
            .sum();

        println!("Movement: {:?}", sub_state);
        println!("Result: {}", sub_state.product());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
