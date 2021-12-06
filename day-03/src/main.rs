use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Diagnostic {
    data: Vec<i32>,
}

impl Diagnostic {
    fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    fn gamma(&self) -> i32 {
        let mut gamma = 0;
        for item in &self.data {
            gamma <<= 1;
            if item > &0 {
                gamma += 1;
            }
        }
        gamma
    }

    fn epsilon(&self) -> i32 {
        let mut epsilon = 0;
        for item in &self.data {
            epsilon <<= 1;
            if item < &0 {
                epsilon += 1;
            }
        }
        epsilon
    }

    fn result(&self) -> i32 {
        self.gamma() * self.epsilon()
    }
}

impl Add for Diagnostic {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result: Vec<i32> = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self::new(result)
    }
}
impl Sum for Diagnostic {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|accum, curr| accum + curr)
            .expect("Error summing Diagnostics")
    }
}

impl FromStr for Diagnostic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<i32> = s
            .chars()
            .flat_map(|c| match c {
                '1' => Ok(1),
                '0' => Ok(-1),
                _ => Err(()),
            })
            .collect();

        Ok(Diagnostic::new(data))
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let diag: Diagnostic = lines.flatten().flat_map(|s| s.parse()).sum();
        println!("Gamma: {}", diag.gamma());
        println!("Epsilon: {}", diag.epsilon());
        println!("Result: {}", diag.result());
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
