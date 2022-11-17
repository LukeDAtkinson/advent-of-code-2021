fn main() {
    let xmin = 137;
    let xmax = 171;
    let ymin = -98;
    let ymax = -73;

    let mut best_vy = 0;
    let mut best_height = 0;
    let mut num_hit = 0;

    for vx in 0..xmax + 1 {
        for vy in ymin..(-ymin) + 1 {
            println!("Launching probe with vx {} and vy {}", vx, vy);
            let mut probe = Probe::new(vx, vy);
            let mut peak = 0;
            while probe.x <= xmax && probe.y >= ymin {
                if probe.y > peak {
                    peak = probe.y;
                    println!("New peak reached by this probe: {}", peak);
                }
                if probe.x >= xmin && probe.x <= xmax && probe.y <= ymax && probe.y >= ymin {
                    println!("Probe reached the target area!");
                    num_hit += 1;
                    if peak > best_height {
                        best_height = peak;
                        best_vy = vy;
                    }
                    break;
                }
                probe = probe.step();
            }
        }
    }

    println!(
        "The highest point reached was {} with starting velocity {}. There were {} different velocity values that hit the target.",
        best_height, best_vy, num_hit
    );
}

struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Probe {
    fn new(vx: i32, vy: i32) -> Probe {
        Probe { x: 0, y: 0, vx, vy }
    }

    fn step(&self) -> Probe {
        Probe {
            x: self.x + self.vx,
            y: self.y + self.vy,
            vx: if self.vx > 0 { self.vx - 1 } else { 0 },
            vy: self.vy - 1,
        }
    }
}
