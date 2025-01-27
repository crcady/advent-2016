use std::io::{self, BufRead};

fn main() {
    let mut robots = vec![Robot::new(); 210];
    let mut val_queue: Vec<(usize, Output)> = Vec::new();
    let mut outputs: Vec<Option<usize>> = vec![None; 24];

    for line in io::stdin().lock().lines() {
        let unwrapped = line.unwrap();
        let splits: Vec<_> = unwrapped.split(" ").collect();
        match splits.len() {
            6 => val_queue.push((
                splits[1].parse().unwrap(),
                Output::Bot(splits[5].parse().unwrap()),
            )),
            12 => {
                let low = match splits[5] {
                    "bot" => Output::Bot(splits[6].parse().unwrap()),
                    "output" => Output::Bin(splits[6].parse().unwrap()),
                    _ => panic!("Unexpected target encountered"),
                };

                let hi = match splits[10] {
                    "bot" => Output::Bot(splits[11].parse().unwrap()),
                    "output" => Output::Bin(splits[11].parse().unwrap()),
                    _ => panic!("Unexpected target encountered"),
                };

                robots[splits[1].parse::<usize>().unwrap()].add_outputs(low, hi)
            }
            _ => panic!("Unexpected line length encountered"),
        }
    }

    while val_queue.len() > 0 {
        let (value, dst) = val_queue.pop().unwrap();
        if let Output::Bot(idx) = dst {
            robots[idx].add_input(value);

            if robots[idx].complete() {
                if robots[idx].input1.unwrap() < robots[idx].input2.unwrap() {
                    val_queue.push((robots[idx].input1.unwrap(), robots[idx].low_out.unwrap()));
                    val_queue.push((robots[idx].input2.unwrap(), robots[idx].hi_out.unwrap()));
                } else {
                    val_queue.push((robots[idx].input2.unwrap(), robots[idx].low_out.unwrap()));
                    val_queue.push((robots[idx].input1.unwrap(), robots[idx].hi_out.unwrap()));
                }
            }
        }

        if let Output::Bin(idx) = dst {
            outputs[idx] = Some(value);
        }
    }

    for i in 0..robots.len() {
        if !robots[i].complete() {
            continue;
        }
        let a = robots[i].input1.unwrap();
        let b = robots[i].input2.unwrap();

        if (a == 17 && b == 61) || (a == 61 && b == 17) {
            println!("Robot {} handles 17 and 61", i);
            break;
        }
    }

    let ans2 = outputs[0].unwrap() * outputs[1].unwrap() * outputs[2].unwrap();
    println!("Multiplying gives {}", ans2);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Output {
    Bot(usize),
    Bin(usize),
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    input1: Option<usize>,
    input2: Option<usize>,
    low_out: Option<Output>,
    hi_out: Option<Output>,
}

impl Robot {
    fn new() -> Self {
        Self {
            input1: None,
            input2: None,
            low_out: None,
            hi_out: None,
        }
    }

    fn add_input(&mut self, value: usize) {
        if self.input1.is_none() {
            self.input1 = Some(value);
        } else {
            assert!(self.input2.is_none(), "Tried to add too many values!");
            self.input2 = Some(value);
        }
    }

    fn add_outputs(&mut self, low: Output, hi: Output) {
        assert!(
            self.low_out.is_none() && self.hi_out.is_none(),
            "Tried to add outputs when they already exist"
        );
        self.low_out = Some(low);
        self.hi_out = Some(hi);
    }

    fn complete(&self) -> bool {
        self.input1.is_some()
            && self.input2.is_some()
            && self.low_out.is_some()
            && self.hi_out.is_some()
    }
}

#[test]
fn test_add_input() {
    let mut r = Robot::new();
    r.add_input(0);
    r.add_input(1);

    assert_eq!(r.input1, Some(0));
    assert_eq!(r.input2, Some(1));
}

#[test]
fn test_add_output() {
    let mut r = Robot::new();
    r.add_outputs(Output::Bot(0), Output::Bot(1));

    assert_eq!(r.low_out, Some(Output::Bot(0)));
    assert_eq!(r.hi_out, Some(Output::Bot(1)));
}

#[test]
fn test_complete() {
    let mut r = Robot::new();
    assert!(!r.complete());

    r.add_input(0);
    r.add_input(1);

    assert!(!r.complete());

    r.add_outputs(Output::Bin(2), Output::Bot(3));

    assert!(r.complete());
}
