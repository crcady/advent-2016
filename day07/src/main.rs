use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let ips: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| IP7::from_line(&x))
        .collect();

    let ans1 = ips
        .iter()
        .filter(|x| x.supports_tls())
        .collect::<Vec<_>>()
        .len();

    let ans2 = ips
        .iter()
        .filter(|x| x.supports_ssl())
        .collect::<Vec<_>>()
        .len();

    println!(
        "Found {} IPv7 addresses that support Transport-layer Snooping",
        ans1
    );

    println!(
        "Found {} IPv7 addresses that support Super-Secret Listening",
        ans2
    );
}
#[derive(Debug)]
struct IP7 {
    regular_sequences: Vec<String>,
    hyper_sequences: Vec<String>,
}

impl IP7 {
    fn from_line(line: &str) -> Self {
        let re = Regex::new(r"\[[a-z]+\]").unwrap();
        let matches: Vec<_> = re.find_iter(line).collect();
        let mut regular_sequences: Vec<String> = Vec::new();
        let mut hyper_sequences: Vec<String> = Vec::new();

        let mut current: usize = 0;
        let end: usize = line.len();

        for m in matches {
            if current < m.start() {
                regular_sequences.push(line[current..m.start()].to_string());
            }

            hyper_sequences.push(line[m.start() + 1..m.end() - 1].to_string());
            current = m.end();
        }

        if current < end {
            regular_sequences.push(line[current..end].to_string());
        }

        Self {
            regular_sequences,
            hyper_sequences,
        }
    }

    fn supports_tls(&self) -> bool {
        let mut regular_count: usize = 0;
        let mut hyper_count: usize = 0;

        for s in &self.regular_sequences {
            let chars: Vec<_> = s.chars().collect();
            for w in chars.windows(4) {
                if w[0] == w[3] && w[1] == w[2] && w[0] != w[1] {
                    regular_count += 1;
                }
            }
        }

        for s in &self.hyper_sequences {
            let chars: Vec<_> = s.chars().collect();
            for w in chars.windows(4) {
                if w[0] == w[3] && w[1] == w[2] && w[0] != w[1] {
                    hyper_count += 1;
                }
            }
        }

        regular_count > 0 && hyper_count == 0
    }

    fn supports_ssl(&self) -> bool {
        let mut seqs: HashSet<String> = HashSet::new();

        for s in &self.regular_sequences {
            let chars: Vec<_> = s.chars().collect();
            for w in chars.windows(3) {
                if w[0] == w[2] && w[0] != w[1] {
                    seqs.insert(String::from_iter(w.iter()));
                }
            }
        }

        for s in &self.hyper_sequences {
            let chars: Vec<_> = s.chars().collect();
            for w in chars.windows(3) {
                if w[0] == w[2] && w[0] != w[1] {
                    let mut s = String::new();
                    s.push(w[1]);
                    s.push(w[0]);
                    s.push(w[1]);

                    if seqs.contains(&s) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[test]
fn test_ip7_create() {
    let ip = IP7::from_line("abba[mnop]qrst");
    dbg!(&ip);
    assert_eq!(ip.regular_sequences.len(), 2);
    assert_eq!(ip.hyper_sequences.len(), 1);
    assert_eq!(&ip.hyper_sequences[0], "mnop")
}

#[test]
fn test_tls() {
    assert!(IP7::from_line("abba[mnop]qrst").supports_tls());
    assert!(!IP7::from_line("abcd[bddb]xyyx").supports_tls());
    assert!(!IP7::from_line("aaaa[qwer]tyui").supports_tls());
    assert!(IP7::from_line("ioxxoj[asdfgh]zxcvbn").supports_tls());
}

#[test]
fn test_ssl() {
    assert!(IP7::from_line("aba[bab]xyz").supports_ssl());
    assert!(!IP7::from_line("xyx[xyx]xyx").supports_ssl());
    assert!(IP7::from_line("aaa[kek]eke").supports_ssl());
    assert!(IP7::from_line("zazbz[bzb]cdb").supports_ssl());
}
