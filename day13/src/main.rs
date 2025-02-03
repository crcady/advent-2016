use std::collections::HashSet;

fn main() {
    let start = Point { x: 1, y: 1 };
    let end = Point { x: 31, y: 39};
    let favorite_number = 1350;

    let steps = search(&start, &end, favorite_number);

    println!("Took {} steps", steps);

    let num_locs = count(&start, favorite_number);

    println!("Can reach {} locations in 50 steps", num_locs);
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_open(&self, num: i32) -> bool {
        let result =
            self.x * self.x + 3 * self.x + 2 * self.x * self.y + self.y + self.y * self.y + num;
        let mut currently_open = true; // We've encountered zero ones, which is even

        // TODO: only looking at 31 bits, assuming the i32 is positive...probably should test for that...
        for i in 0..31 {
            if (result >> i) & 1 != 0 {
                currently_open = !currently_open;
            }
        }
        currently_open
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut res = Vec::new();
        res.push(Point {
            x: self.x + 1,
            y: self.y,
        });
        res.push(Point {
            x: self.x,
            y: self.y + 1,
        });
        if self.x > 0 {
            res.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.y > 0 {
            res.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }

        res
    }
}

#[test]
fn test_currently_open() {
    for row in 0..7 {
        for col in 0..10 {
            let p = Point { x: col, y: row };
            if p.is_open(10) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}

fn search(start: &Point, end: &Point, favorite_number: i32) -> usize {
    let mut frontier = HashSet::new();
    frontier.insert(*start);

    let mut seen = HashSet::new();
    let mut steps = 0;

    'outer: loop {
        steps += 1;
        let mut next_frontier = HashSet::new();
        for current in frontier.iter() {
            seen.insert(*current);
            for p in current.neighbors() {
                if !p.is_open(favorite_number) {
                    continue;
                }
                if &p == end {
                    break 'outer;
                }
                if !seen.contains(&p) {
                    next_frontier.insert(p);
                }
            }
        }
        frontier = next_frontier;
    }

    steps
}

#[test]
fn test_search() {
    assert_eq!(search(&Point { x: 1, y: 1 }, &Point { x: 7, y: 4 }, 10), 11);
}

fn count(start: &Point, favorite_number: i32) -> usize {
    let mut frontier = HashSet::new();
    frontier.insert(*start);

    let mut seen = HashSet::new();

    for _ in 0..51 {
        let mut next_frontier = HashSet::new();
        for current in frontier.iter() {
            seen.insert(*current);
            for p in current.neighbors() {
                if !p.is_open(favorite_number) {
                    continue;
                }

                if !seen.contains(&p) {
                    next_frontier.insert(p);
                }
            }
        }
        frontier = next_frontier;
    }

    seen.len()
}