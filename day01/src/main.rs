use std::{collections::HashSet, io};

fn main() {
    let mut me = Me::new();
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let directions: Vec<String> = line.split(", ").map(|x| x.to_string()).collect();

    for dir in &directions {
        me.walk(dir);
    }
    println!("Total distance away is {}", me.distance());

    let mut me = Me::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    
    'outer: for dir in &directions {
        for p in me.walk(dir) {
            let new_loc = visited.insert(p);
            if !new_loc {
                let (x, y) = p;
                me.x = x;
                me.y = y;
                break 'outer;
            }
        }
    }
    println!("Found a new location {} away", me.distance());
}

struct Me {
    x: i32,
    y: i32,
    direction: char,
}

impl Me {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: 'N',
        }
    }

    fn walk(&mut self, directions: &str) -> Vec<(i32, i32)> {
        let mut iter = directions.chars();
        let turn_dir = iter.next().unwrap();
        let how_far: i32 = String::from_iter(iter).parse().unwrap();
        match turn_dir {
            'R' => {
                self.direction = match self.direction {
                    'N' => 'E',
                    'E' => 'S',
                    'S' => 'W',
                    'W' => 'N',
                    _ => unimplemented!(),
                }
            }
            'L' => {
                self.direction = match self.direction {
                    'N' => 'W',
                    'W' => 'S',
                    'S' => 'E',
                    'E' => 'N',
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }

        let mut res: Vec<(i32, i32)> = Vec::new();
        for _ in 0..how_far {
            match self.direction {
                'N' => self.y += 1,
                'E' => self.x += 1,
                'S' => self.y -= 1,
                'W' => self.x -= 1,
                _ => unimplemented!(),
            }
            res.push((self.x, self.y))
        }
        res
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[test]
fn test_walk() {
    let mut me = Me::new();
    me.walk("R2");
    assert_eq!(me.x, 2);
    assert_eq!(me.y, 0);
    me.walk("L3");
    assert_eq!(me.x, 2);
    assert_eq!(me.y, 3);
}

#[test]
fn test_distance() {
    let mut me = Me::new();
    me.walk("R2");
    me.walk("L3");
    assert_eq!(me.distance(), 5);
}

#[test]
fn test_trail() {
    let mut me = Me::new();
    assert_eq!(me.walk("R4"), vec![(1, 0), (2, 0), (3, 0), (4, 0)]);
}
