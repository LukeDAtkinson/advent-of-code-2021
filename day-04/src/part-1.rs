use std::{fs, str::FromStr};

fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("./input.txt").expect("Failed to read file contents to string");

    let mut inputs = input.split("\n\n");
    let first_line = inputs.next().expect("");
    let mut boards: Vec<Board> = inputs
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("Failed to parse string {s}"))
        .collect();

    let result = first_line
        .split(',')
        .map(|s| s.parse::<u32>().expect("Failed to parse input number"))
        .find_map(|n| {
            boards = boards.iter().map(|board| board.announce(n)).collect();
            boards
                .iter()
                .find_map(|board| board.check())
                .map(|score| score * n)
        });
    
    println!("result: {}", result.unwrap() );
}

#[derive(Copy, Clone)]
enum Field {
    MARKED(u32),
    UNMARKED(u32),
}

impl Field {
    fn announce(self, number: u32) -> Field {
        match self {
            Field::UNMARKED(n) if n == number => Field::MARKED(n),
            _ => self,
        }
    }
}

struct Board {
    numbers: Vec<Vec<Field>>,
}

impl Board {
    fn announce(&self, number: u32) -> Board {
        Board {
            numbers: self
                .numbers
                .iter()
                .map(|row| row.iter().map(|field| field.announce(number)).collect())
                .collect(),
        }
    }

    fn check(&self) -> Option<u32> {
        let complete_row = self.numbers.iter().any(|row| {
            row.iter().all(|f| match f {
                Field::MARKED(_) => true,
                Field::UNMARKED(_) => false,
            })
        });
        if complete_row {
            return Option::Some(self.score());
        }

        let mut complete_column;
        let len = self.numbers[0].len();
        for i in 0..len {
            complete_column = self.numbers.iter().all(|row| match row[i] {
                Field::MARKED(_) => true,
                Field::UNMARKED(_) => false,
            });
            if complete_column {
                return Option::Some(self.score());
            }
        }

        Option::None
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|f| match f {
                        Field::MARKED(_) => None,
                        Field::UNMARKED(n) => Some(n),
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("Attempting to parse {} to board", s);
        let numbers: Vec<Vec<Field>> = s
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .map(|val| val.parse().expect("Failed to parse val {val}"))
                    .map(|number| Field::UNMARKED(number))
                    .collect()
            })
            .collect();

        Ok(Self { numbers })
    }
}
