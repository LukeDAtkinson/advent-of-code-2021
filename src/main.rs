use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String

        let mut last_three: VecDeque<Option<i32>> = VecDeque::from([None, None, None]);
        let mut count = 0;
        for current in lines.flatten() {
            let cur: i32 = current.parse().expect("Failed to parse depth");
            let prev = last_three.pop_front().flatten();
            if let Some(p) = prev {
                if cur > p {
                    count += 1;
                }
            }
            last_three.push_back(Some(cur))
        }
        println!("{}", count);
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
