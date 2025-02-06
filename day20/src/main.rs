use std::io::{self, BufRead};

fn main() {
    let mut solver = Solver::new(4294967295);

    for line in io::stdin().lock().lines() {
        solver.add_range(BlockRange::from_line(&line.unwrap()));
    }

    println!("The first allowed IP is {}", solver.get_first_allowed());
    println!("There are {} IPs allowed", solver.count_allowed());
}

struct BlockRange {
    start: u64,
    end: u64,
}

impl BlockRange {
    fn from_line(line: &str) -> Self {
        let splits: Vec<_> = line.split("-").collect();
        let start: u64 = splits[0].parse().unwrap();
        let end: u64 = splits[1].parse().unwrap();

        Self { start, end }
    }
}

struct Solver {
    ranges: Vec<BlockRange>,
    max_val: u64,
}

impl Solver {
    fn new(max_val: u64) -> Self {
        Self {
            ranges: Vec::new(),
            max_val,
        }
    }

    fn add_range(&mut self, br: BlockRange) {
        self.ranges.push(br);
        self.ranges.sort_by_key(|x| x.start);
    }

    fn get_first_allowed(&self) -> u64 {
        let mut first = 0;
        for current in &self.ranges {
            if first < current.start {
                return first;
            }

            if current.end + 1 > first {
                first = current.end + 1;
            }
        }
        first
    }

    fn count_allowed(&self) -> u64 {
        let mut count = 0;
        let mut next = 0;
        for current in &self.ranges {
            if next < current.start {
                count += current.start - next;
            }

            if current.end + 1 > next {
                next = current.end + 1;
            }
        }

        if next <= self.max_val {
            count += (self.max_val - next) + 1;
        }

        count
    }
}
#[test]
fn test_solver() {
    let mut solver = Solver::new(9);
    solver.add_range(BlockRange::from_line("5-8"));
    solver.add_range(BlockRange::from_line("0-2"));
    solver.add_range(BlockRange::from_line("4-7"));

    assert_eq!(solver.get_first_allowed(), 3);
    assert_eq!(solver.count_allowed(), 2);
}
