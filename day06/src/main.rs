use std::{collections::HashMap, io::{self, BufRead}};

fn main() {
    let mut r = Repeater::new();
    for line in io::stdin().lock().lines() {
        r.add_line(&line.unwrap());
    }

    println!("The secret message is {}", r.message());
    println!("The other message is {}", r.message2());
}

struct Repeater {
    freqs: Vec<HashMap<char, usize>>,
}

impl Repeater {
    fn new() -> Self {
        Self {
            freqs: Vec::new(),
        }
    }

    fn add_line(&mut self, line: &str) {
        if self.freqs.len() == 0 {
            for _ in 0..line.len() {
                self.freqs.push(HashMap::new());
            }
        } else {
            assert_eq!(self.freqs.len(), line.len(), "Line added with too few characters");
        }

        let mut it = line.chars();

        for i in 0..line.len() {
            let c = it.next().unwrap();
            let old_val = *self.freqs[i].get(&c).unwrap_or(&0);

            self.freqs[i].insert(
                c,
                old_val+1
            );
        }
    }

    fn message(&self) -> String {
        let mut res = String::new();
        for i in 0..self.freqs.len() {
            let mut max_num: usize = 0;
            let mut the_ch: char = ' ';
            for (ch, num) in self.freqs[i].iter() {
                if *num > max_num {
                    the_ch = *ch;
                    max_num = *num;
                }
            }

            res.push(the_ch);
        }

        res
    }

    fn message2(&self) -> String {
        let mut res = String::new();
        for i in 0..self.freqs.len() {
            let mut it = self.freqs[i].iter();
            let mut min_num: usize;
            let mut the_ch: char;
            let (k, v) = it.next().unwrap();
            min_num = *v;
            the_ch = *k;

            for (k, v) in it {
                if *v < min_num {
                    the_ch = *k;
                    min_num = *v;
                }
            }
            res.push(the_ch);
        }

        res
    }
}

#[test]
fn test_repeater() {
    let mut r = Repeater::new();
    r.add_line("secret");
    assert_eq!(&r.message(), "secret");
    assert_eq!(&r.message2(), "secret");
}