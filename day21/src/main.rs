use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let mut pw: Password = "abcdefgh".chars().collect();
    let mut ops = Vec::new();
    for line in io::stdin().lock().lines() {
        let op = Operation::from_line(&line.unwrap());
        ops.push(op);
    }
    for op in &ops {
        apply(&mut pw, op);
    }
    let final_pw = String::from_iter(pw.iter());
    println!("The password is {}", final_pw);

    let pw: Password = "abcdefgh".chars().collect();
    for perm in pw.into_iter().permutations(8) {
        let mut pw = perm.clone();
        for op in &ops {
            apply(&mut pw, op);
        }
        if pw == "fbgdceah".chars().collect::<Vec<_>>() {
            let good_input = String::from_iter(perm.iter());
            println!("Unscrambled is {}", good_input);
            break;
        }
    }
}

enum Operation {
    SwapPos(usize, usize),
    SwapLet(char, char),
    RotLeft(usize),
    RotRight(usize),
    RotPos(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn from_line(line: &str) -> Self {
        let splits: Vec<_> = line.split(" ").collect();
        match splits[0] {
            "swap" => match splits[1] {
                "position" => {
                    Operation::SwapPos(splits[2].parse().unwrap(), splits[5].parse().unwrap())
                }
                "letter" => Operation::SwapLet(
                    splits[2].chars().next().unwrap(),
                    splits[5].chars().next().unwrap(),
                ),
                _ => unreachable!(),
            },
            "rotate" => match splits[1] {
                "left" => Operation::RotLeft(splits[2].parse().unwrap()),
                "right" => Operation::RotRight(splits[2].parse().unwrap()),
                "based" => Operation::RotPos(splits[6].chars().next().unwrap()),
                _ => unreachable!(),
            },
            "reverse" => Operation::Reverse(splits[2].parse().unwrap(), splits[4].parse().unwrap()),
            "move" => Operation::Move(splits[2].parse().unwrap(), splits[5].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

type Password = Vec<char>;

fn apply(pw: &mut Password, op: &Operation) {
    match op {
        Operation::SwapPos(x, y) => {
            let tmp = pw[*x];
            pw[*x] = pw[*y];
            pw[*y] = tmp;
        }
        Operation::SwapLet(x, y) => {
            for i in 0..pw.len() {
                if pw[i] == *x {
                    pw[i] = *y;
                } else if pw[i] == *y {
                    pw[i] = *x;
                }
            }
        }
        Operation::RotLeft(count) => {
            for _ in 0..*count {
                let tmp = pw[0];
                for i in 0..pw.len() - 1 {
                    pw[i] = pw[i + 1];
                }
                let idx = pw.len() - 1;
                pw[idx] = tmp;
            }
        }
        Operation::RotRight(count) => {
            for _ in 0..*count {
                let end_idx = pw.len() - 1;
                let tmp = pw[end_idx];
                for i in 0..pw.len() - 1 {
                    pw[end_idx - i] = pw[end_idx - i - 1];
                }
                pw[0] = tmp;
            }
        }
        Operation::RotPos(x) => {
            let mut idx = 0;
            while pw[idx] != *x {
                idx += 1;
            }

            let mut rot_amt = 1 + idx;
            if idx >= 4 {
                rot_amt += 1;
            }

            let op = Operation::RotRight(rot_amt);
            apply(pw, &op);
        }
        Operation::Reverse(start, end) => {
            let mut tmp = Vec::with_capacity(end - start + 1);
            for i in *start..=*end {
                tmp.push(pw[i]);
            }

            for i in *start..=*end {
                pw[i] = tmp.pop().unwrap();
            }
        }
        Operation::Move(x, y) => {
            if x < y {
                let tmp = pw[*x];
                for i in *x..*y {
                    pw[i] = pw[i + 1];
                }
                pw[*y] = tmp;
            }

            if x > y {
                let tmp = pw[*x];
                for i in 0..(*x - *y) {
                    pw[*x - i] = pw[*x - i - 1];
                }
                pw[*y] = tmp;
            }
        }
    };
}

#[test]
fn test_apply() {
    let mut pw: Password = "abcde".chars().collect();
    let op = Operation::SwapPos(4, 0);
    apply(&mut pw, &op);
    assert_eq!(pw, "ebcda".chars().collect::<Vec<_>>());

    let op = Operation::SwapLet('d', 'b');
    apply(&mut pw, &op);
    assert_eq!(pw, "edcba".chars().collect::<Vec<_>>());

    let op = Operation::Reverse(0, 4);
    apply(&mut pw, &op);
    assert_eq!(pw, "abcde".chars().collect::<Vec<_>>());

    let op = Operation::RotLeft(1);
    apply(&mut pw, &op);
    assert_eq!(pw, "bcdea".chars().collect::<Vec<_>>());

    let op = Operation::Move(1, 4);
    apply(&mut pw, &op);
    assert_eq!(pw, "bdeac".chars().collect::<Vec<_>>());

    let op = Operation::Move(3, 0);
    apply(&mut pw, &op);
    assert_eq!(pw, "abdec".chars().collect::<Vec<_>>());

    let op = Operation::RotPos('b');
    apply(&mut pw, &op);
    assert_eq!(pw, "ecabd".chars().collect::<Vec<_>>());

    let op = Operation::RotPos('d');
    apply(&mut pw, &op);
    assert_eq!(pw, "decab".chars().collect::<Vec<_>>());
}
