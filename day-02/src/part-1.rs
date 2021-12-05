use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Movement {
    x: i32,
    y: i32,
}

impl Movement {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn product(self) -> i32 {
        self.x * self.y
    }
}

impl Add for Movement {
    type Output = Movement;

    fn add(self, rhs: Self) -> Self::Output {
        Movement::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sum for Movement {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|accum, it| accum + it)
            .expect("Error summing movements")
    }
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(' ');
        let (direction, dist_str) = split.expect("Failed to split input string");
        let distance: i32 = dist_str.parse().expect("Failed to parse distance");

        match direction {
            "forward" => Ok(Movement::new( distance, 0 )),
            "up" => Ok(Movement::new( 0, -distance )),
            "down" => Ok(Movement::new( 0, distance )),
            _ => Err(()),
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let sum_movement: Movement = lines
            .flatten()
            .map(|instruction| Movement::from_str(instruction.as_str()))
            .flatten()
            .sum();

        println!("Movement: {:?}", sum_movement);
        println!("Result: {}", sum_movement.product());
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
