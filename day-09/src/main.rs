use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct World {
    depths: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl World {
    fn new(depths: Vec<Vec<u8>>) -> Self {
        World {
            width: depths.get(0).unwrap().len(),
            height: depths.len(),
            depths,
        }
    }

    fn depth(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 || y as usize >= self.depths.len() {
            return None;
        }
        let row = self.depths.get(y as usize).unwrap();
        if x as usize >= row.len() {
            return None;
        }
        row.get(x as usize).map(|d| *d)
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let world = World::new(
            lines
                .flatten()
                .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
                .collect(),
        );

        println!("World: {:?}", world);

        let mut total: usize = 0;
        for i in 0..world.width {
            for j in 0..world.height {
                let depth = world.depth(i as isize, j as isize).unwrap();

                if [
                    world.depth(i as isize - 1, j as isize),
                    world.depth(i as isize + 1, j as isize),
                    world.depth(i as isize, j as isize - 1),
                    world.depth(i as isize, j as isize + 1),
                ]
                .iter()
                .flatten()
                .all(|neighbour| depth < *neighbour)
                {
                    total += depth as usize + 1;
                }
            }
        }

        println!("Total: {}", total);
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
