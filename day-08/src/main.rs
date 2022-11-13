use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct Mapping {
    chars: HashSet<char>,
    number: i8,
}

impl Mapping {
    fn new(chars: HashSet<char>, number: i8) -> Self {
        Mapping { chars, number }
    }
}

#[derive(Debug)]
struct Cipher {
    mappings: Vec<Mapping>,
}

impl Cipher {
    fn new(s: &str) -> Self {
        let inputs = Self::parse_space_separated_string(s);

        let one = inputs.iter().find(|i| i.len() == 2).unwrap();
        let four = inputs.iter().find(|i| i.len() == 4).unwrap();
        let seven = inputs.iter().find(|i| i.len() == 3).unwrap();
        let eight = inputs.iter().find(|i| i.len() == 7).unwrap();

        let cde = inputs
            .iter()
            .filter(|i| i.len() == 6)
            .flat_map(|i| eight - i)
            .collect::<HashSet<char>>();

        let e = &cde - one;

        let two = inputs
            .iter()
            .find(|i| i.len() == 5 && i.is_superset(&e))
            .unwrap();
        let three = inputs
            .iter()
            .find(|i| i.len() == 5 && i.is_superset(one))
            .unwrap();
        let five = inputs
            .iter()
            .find(|i| i.len() == 5 && i != &two && i != &three)
            .unwrap();

        let nine = inputs
            .iter()
            .find(|i| i.len() == 6 && i.is_superset(four))
            .unwrap();
        let zero = inputs
            .iter()
            .find(|i| i.len() == 6 && i.is_superset(one) && i != &nine)
            .unwrap();
        let six = inputs
            .iter()
            .find(|i| i.len() == 6 && i != &nine && i != &zero)
            .unwrap();

        let mappings = vec![
            Mapping::new(zero.to_owned(), 0),
            Mapping::new(one.to_owned(), 1),
            Mapping::new(two.to_owned(), 2),
            Mapping::new(three.to_owned(), 3),
            Mapping::new(four.to_owned(), 4),
            Mapping::new(five.to_owned(), 5),
            Mapping::new(six.to_owned(), 6),
            Mapping::new(seven.to_owned(), 7),
            Mapping::new(eight.to_owned(), 8),
            Mapping::new(nine.to_owned(), 9),
        ];

        Cipher { mappings }
    }

    fn score(&self, input: &str) -> usize {
        let inputs = Self::parse_space_separated_string(input);

        println!("Separated inputs: {:?}", inputs);

        inputs
            .iter()
            .map(|i| self.value(i).expect("Something's broken") as usize)
            .zip([1000, 100, 10, 1])
            .map(|(a, b)| a * b)
            .sum()
    }

    fn parse_space_separated_string(input: &str) -> Vec<HashSet<char>> {
        let inputs: Vec<HashSet<char>> = input
            .trim()
            .split(' ')
            .map(|s| -> HashSet<char> { s.trim().chars().collect() })
            .collect();
        inputs
    }

    fn value(&self, input: &HashSet<char>) -> Result<i8, &str> {
        println!("Mapping input: {:?}", input);
        for mapping in &self.mappings {
            println!("Checking cipher mapping: {:?}", mapping);
            if mapping.chars == *input {
                return Ok(mapping.number);
            };
        }
        Err("Couldn't match input set to a cipher value")
    }
}

fn score(line: String) -> usize {
    let split = line.split_once('|').unwrap();
    println!("Building cipher with input string: {:?}", split.0);
    let cipher = Cipher::new(split.0);
    println!("Cipher: {:?}", cipher);

    println!("Using cipher to decode: {:?}", split.1);
    cipher.score(split.1)
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let result: usize = lines.flatten().map(|s| score(s)).sum();

        println!("Result: {}", result);
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
