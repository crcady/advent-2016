use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let mut it = io::stdin().lock().lines();
    it.next();
    it.next();

    let mut nodes: Vec<Node> = it.map(|x| Node::from_line(&x.unwrap())).collect();

    let mut paircount = 0;

    let mut goal_index = 0;

    for i in 0..nodes.len() {
        if nodes[i].y == 0 && nodes[i].x > nodes[goal_index].x {
            goal_index = i;
        }

        for j in 0..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];

            if a.compare(b) {
                paircount += 1;
            }
        }
    }

    println!("Found {} pairs", paircount);
    dbg!(goal_index);
    nodes[goal_index].is_goal = true;

    let steps = breadth_first_search(&nodes);

    println!("Took {} steps", steps);
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Node {
    x: i8,
    y: i8,
    used: i16,
    available: i16,
    is_goal: bool,
}

impl Node {
    fn from_line(line: &str) -> Self {
        let splits: Vec<_> = line.split_whitespace().collect();
        let coords: Vec<_> = splits[0].split("-").collect();

        let x: i8 = coords[1][1..].parse().unwrap();
        let y: i8 = coords[2][1..].parse().unwrap();

        let make_i16 = |s: &str| s[..s.len() - 1].parse::<i16>().unwrap();

        let _size = make_i16(splits[1]);
        let used = make_i16(splits[2]);
        let available = make_i16(splits[3]);
        let _percent = make_i16(splits[4]);

        Self {
            x,
            y,
            //size,
            used,
            available,
            //percent,
            is_goal: false,
        }
    }

    fn compare(&self, b: &Self) -> bool {
        if self.used == 0 {
            return false;
        }

        if self.x == b.x && self.y == b.y {
            return false;
        }

        self.used <= b.available
    }

    fn empty_into(&mut self, b: &mut Self) {
        assert!(self.compare(b), "Cannot empty into a bad pairing");
        b.used += self.used;
        b.available -= self.used;
        self.available += self.used;
        self.used = 0;

        if self.is_goal {
            b.is_goal = true;
            self.is_goal = false;
        }
    }
}

fn breadth_first_search(nodes: &Vec<Node>) -> usize {
    let mut generations = 0;
    let mut frontier: HashSet<Vec<Node>> = HashSet::new();
    let mut already_seen = HashSet::new();
    frontier.insert(nodes.clone());

    'outer: loop {
        println!("Generation {}: {} candidates", generations, frontier.len());
        let mut next_frontier = HashSet::new();
        for node_vec in frontier {
            if node_vec[0].is_goal {
                break 'outer;
            }

            let mut i = 0;
            while node_vec[i].used != 0 {
                i += 1;
            }

            for j in 0..node_vec.len() {
                let dx = (node_vec[i].x - node_vec[j].x).abs();
                let dy = (node_vec[i].y - node_vec[j].y).abs();
                if dx + dy != 1 {
                    continue;
                }

                if node_vec[j].compare(&node_vec[i]) {
                    let mut new_vec = node_vec.clone();
                    if i < j {
                        let (left, right) = new_vec.split_at_mut(i + 1);
                        right[j - i - 1].empty_into(&mut left[i]);
                    } else {
                        let (left, right) = new_vec.split_at_mut(j + 1);
                        left[j].empty_into(&mut right[i - j - 1]);
                    }
                    // new_vec[j].empty_into(&mut new_vec[i]); <--- Illegal!
                    if !already_seen.contains(&new_vec) {
                        next_frontier.insert(new_vec);
                    }
                }
            }
            already_seen.insert(node_vec);
        }
        generations += 1;
        frontier = next_frontier;
    }

    generations
}

#[test]
fn test_from_line() {
    let line = "/dev/grid/node-x0-y0     85T   64T    21T   75%";
    let node = Node::from_line(line);

    assert_eq!(node.x, 0);
    assert_eq!(node.y, 0);
    //assert_eq!(node.size, 85);
    assert_eq!(node.used, 64);
    assert_eq!(node.available, 21);
    //assert_eq!(node.percent, 75);
}
