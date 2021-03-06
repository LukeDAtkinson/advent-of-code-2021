use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut scores: Vec<u64> = lines
            .flatten()
            .map(|s| s.chars().collect::<Vec<char>>())
            .map(|cs| compute_autocomplete_score(cs))
            .flatten()
            .collect();
        println!("scores: {:?}", scores);
        scores.sort();
        println!("scores: {:?}", scores);
        let result = scores.get(scores.len() / 2);
        println!("result: {}", result.unwrap());
    }
}

fn compute_autocomplete_score(chars: Vec<char>) -> Option<u64> {
    let mut parse_queue = VecDeque::new();
    let result = chars.iter().find_map(|c| match c {
        '(' | '{' | '[' | '<' => {
            parse_queue.push_front(c);
            None
        }
        ')' => {
            let prev = parse_queue.pop_front();
            if prev == Some(&'(') {
                None
            } else {
                Some(3)
            }
        }
        ']' => {
            let prev = parse_queue.pop_front();
            if prev == Some(&'[') {
                None
            } else {
                Some(57)
            }
        }
        '}' => {
            let prev = parse_queue.pop_front();
            if prev == Some(&'{') {
                None
            } else {
                Some(1197)
            }
        }
        '>' => {
            let prev = parse_queue.pop_front();
            if prev == Some(&'<') {
                None
            } else {
                Some(25137)
            }
        }
        _ => None,
    });
    println!("Ending parse queue: {:?}", parse_queue);
    if result.is_some() {
        None
    } else {
        let mut score: u64 = 0;
        while let Some(c) = parse_queue.pop_front() {
            score *= 5;
            score += match c {
                &'(' => 1,
                &'[' => 2,
                &'{' => 3,
                &'<' => 4,
                &_ => 0,
            }
        }
        Some(score)
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
