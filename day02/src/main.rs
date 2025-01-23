use std::{
    cmp::{max, min},
    io::{self, BufRead},
};

fn main() {
    let mut code = String::new();
    let mut code2 = String::new();
    let mut current_kp = KeyPad::new();
    let mut current_dp = DiamondPad::new();

    for line in io::stdin().lock().lines() {
        let unwrapped = line.unwrap();
        current_kp = unwrapped
            .chars()
            .fold(current_kp, |kp, c| kp.handle_char(&c));

        current_dp = unwrapped
            .chars()
            .fold(current_dp, |dp, c| dp.handle_char(&c));
        
        code += &current_kp.code();
        code2 += &current_dp.code();

    }
    println!("Code: {}", &code);
    println!("Second code: {}", &code2)
}

#[derive(Debug)]
struct KeyPad {
    x: i8,
    y: i8,
}

#[derive(Debug, Clone, Copy)]
struct DiamondPad {
    x: i8,
    y: i8,
}

impl KeyPad {
    fn new() -> Self {
        Self { x: 1, y: 1 }
    }

    fn handle_char(&self, c: &char) -> Self {
        match c {
            'U' => Self {
                y: clamp3(self.y - 1),
                ..*self
            },
            'D' => Self {
                y: clamp3(self.y + 1),
                ..*self
            },
            'L' => Self {
                x: clamp3(self.x - 1),
                ..*self
            },
            'R' => Self {
                x: clamp3(self.x + 1),
                ..*self
            },
            _ => panic!(),
        }
    }

    fn code(&self) -> String {
        let num = 3 * self.y + self.x + 1;
        num.to_string()
    }
}

impl DiamondPad {
    fn new() -> Self {
        Self { x: -2, y: 0 }
    }

    fn handle_char(&self, c: &char) -> Self {
        let candidate = match c {
            'U' => Self {
                y: self.y - 1,
                ..*self
            },
            'D' => Self {
                y: self.y + 1,
                ..*self
            },
            'L' => Self {
                x: self.x - 1,
                ..*self
            },
            'R' => Self {
                x: self.x + 1,
                ..*self
            },
            _ => panic!(),
        };

        if candidate.x.abs() + candidate.y.abs() <= 2 {
            candidate
        } else {
            *self
        }
    }

    fn code(&self) -> String {
        let lookup_table: Vec<Vec<&'static str>> = vec![
            vec!["", "", "1", "", ""],
            vec!["", "2", "3", "4", ""],
            vec!["5", "6", "7", "8", "9"],
            vec!["", "A", "B", "C", ""],
            vec!["", "", "D", "", ""],
        ];
        lookup_table[(self.y + 2) as usize][(self.x + 2) as usize].to_string()
    }
}

fn clamp3(num: i8) -> i8 {
    max(0, min(num, 2))
}

#[test]
fn test_clamp() {
    assert_eq!(clamp3(-1), 0);
    assert_eq!(clamp3(0), 0);
    assert_eq!(clamp3(2), 2);
    assert_eq!(clamp3(3), 2);
}

#[test]
fn test_handle() {
    let kp = KeyPad::new();
    assert_eq!(kp.x, 1);
    assert_eq!(kp.y, 1);
    let l = kp.handle_char(&'L');
    assert_eq!(l.x, 0);
    assert_eq!(l.y, 1);
    let ll = l.handle_char(&'L');
    assert_eq!(ll.x, 0);
    assert_eq!(ll.y, 1);
}

#[test]
fn test_code() {
    let kp = KeyPad::new();
    assert_eq!(kp.code(), "5");
}

#[test]
fn test_dp() {
    let dp = DiamondPad::new();
    assert_eq!(dp.x, -2);
    assert_eq!(dp.y, 0);
    assert_eq!(dp.code(), "5");

    let l = dp.handle_char(&'L');
    assert_eq!(l.code(), "5");

    let r = dp.handle_char(&'R');
    assert_eq!(r.code(), "6");

    let rd = r.handle_char(&'D');
    assert_eq!(rd.code(), "A");
}
