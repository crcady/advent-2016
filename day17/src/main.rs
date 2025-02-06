fn main() {
    let mut frontier = Vec::new();
    frontier.push(PathTracker::new("ihgpwlah"));
    'outer: loop {
        let mut next_frontier = Vec::new();
        for pt in frontier {
            if pt.row == 3 && pt.col == 3 {
                println!("The best path is {}", pt.path);
                break 'outer;
            }
            next_frontier.append(&mut pt.get_next());
        }
        frontier = next_frontier;
    }

    let mut frontier = Vec::new();
    frontier.push(PathTracker::new("ihgpwlah"));
    let mut max_len = 0;

    while frontier.len() > 0 {
        let mut next_frontier = Vec::new();
        for pt in frontier {
            if pt.row == 3 && pt.col == 3 {
                if pt.path.len() > max_len {
                    max_len = pt.path.len();
                }
                continue;
            }
            next_frontier.append(&mut pt.get_next());
        }
        frontier = next_frontier;
    }
    println!("The longest path is {} steps", max_len);

}

struct PathTracker {
    path: String,
    passcode: String,
    row: u8,
    col: u8,
}

impl PathTracker {
    fn new(pass: &str) -> Self {
        Self {
            path: "".to_string(),
            passcode: pass.to_string(),
            row: 0,
            col: 0,
        }
    }

    fn get_next(&self) -> Vec<Self> {
        let data = [self.passcode.to_string(), self.path.to_string()].concat();
        let hash = format!("{:x}", md5::compute(data));
        let mut it = hash.chars();

        let up = match it.next().unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => true,
            _ => false,
        };

        let down = match it.next().unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => true,
            _ => false,
        };

        let left = match it.next().unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => true,
            _ => false,
        };

        let right = match it.next().unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => true,
            _ => false,
        };

        let mut res = Vec::new();
        if up && self.row > 0 {
            res.push(Self {
                path: self.path.to_string() + "U",
                passcode: self.passcode.to_string(),
                row: self.row - 1,
                ..*self
            });
        }

        if down && self.row < 3 {
            res.push(Self {
                path: self.path.to_string() + "D",
                passcode: self.passcode.to_string(),
                row: self.row + 1,
                ..*self
            });
        }

        if left && self.col > 0 {
            res.push(Self {
                path: self.path.to_string() + "L",
                passcode: self.passcode.to_string(),
                col: self.col - 1,
                ..*self
            });
        }

        if right && self.col < 3 {
            res.push(Self {
                path: self.path.to_string() + "R",
                passcode: self.passcode.to_string(),
                col: self.col + 1,
                ..*self
            });
        }

        res
    }
}

#[test]
fn test_next() {
    let pt = PathTracker::new("hijkl");
    let mut n = pt.get_next();
    assert_eq!(n.len(), 1);
    let pt = n.pop().unwrap();
    assert_eq!(pt.path, "D");
    let n = pt.get_next();
    assert_eq!(n.len(), 2);
}
