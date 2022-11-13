use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let total: i32 = lines
            .flatten()
            .map(|s| s.chars().collect::<Vec<char>>())
            .map(|chs| compute_syntax_error_score(chs))
            .flatten()
            .sum();
        println!("Total: {}", total);
    }
}

fn compute_syntax_error_score(chars: Vec<char>) -> Option<i32> {
    let mut parse_queue = VecDeque::new();
    chars.iter().find_map(|c| match c {
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
    })
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
