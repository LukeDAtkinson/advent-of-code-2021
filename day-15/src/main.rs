use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
    fs::File,
    io::{self, BufRead},
    path::Path,
    slice::Iter,
};

fn main() {
    let input: Vec<Vec<usize>> = read_lines("./input")
        .expect("Failed to read input")
        .map(|line| line.expect("Failed to read line in input"))
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Non numerical char encountered."))
                .map(|it| it as usize)
                .collect()
        })
        .collect();

    let cave = Cave { map: input };

    let route = Route {
        point: Point2d { x: 0, y: 0 },
        risk: 0,
    };

    let mut to_eval = BinaryHeap::new();
    to_eval.push(route);

    let mut visited: HashSet<Point2d> = HashSet::new();

    let destination = Point2d::new(cave.width() - 1, cave.height() - 1);

    while !to_eval.is_empty() {
        let here = to_eval.pop().unwrap();
        if here.point == destination {
            println!("Total risk: {}", here.risk);
            break;
        } else if !visited.contains(&here.point) {
            here.point
                .neighbours()
                .iter()
                .filter(|p| p.x < cave.width() && p.y < cave.height())
                .map(|p| Route::new(*p, here.risk + cave.get(p)))
                .for_each(|r| to_eval.push(r));
            visited.insert(here.point);
        }
    }
}

struct Cave {
    map: Vec<Vec<usize>>,
}

impl Cave {
    fn get(&self, coord: &Point2d) -> usize {
        let risk = self
            .map
            .get(coord.x % self.map.len())
            .expect("x coordinate out of bounds")
            .get(coord.y % self.map[0].len())
            .expect("y coordinate out of bounds");

        let dx = coord.x / self.map.len();
        let dy = coord.y / self.map[0].len();
        let summed_risk = risk + dx + dy;
        let modulo_risk = if summed_risk > 9 {
            summed_risk % 10 + 1
        } else {
            summed_risk
        };
        modulo_risk
    }

    fn width(&self) -> usize {
        self.map.len() * 5
    }

    fn height(&self) -> usize {
        self.map[0].len() * 5
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
struct Point2d {
    x: usize,
    y: usize,
}

impl Point2d {
    fn new(x: usize, y: usize) -> Point2d {
        Point2d { x, y }
    }

    fn neighbours(&self) -> Vec<Point2d> {
        let mut result: Vec<Point2d> = Vec::new();
        result.push(Point2d::new(self.x + 1, self.y));
        result.push(Point2d::new(self.x, self.y + 1));
        if self.x > 0 {
            result.push(Point2d::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            result.push(Point2d::new(self.x, self.y - 1));
        }
        result
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Route {
    point: Point2d,
    risk: usize,
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.risk.partial_cmp(&other.risk).map(|o| o.reverse())
    }
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

impl Route {
    fn new(point: Point2d, risk: usize) -> Route {
        Route { point, risk }
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
