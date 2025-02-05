use std::collections::HashSet;

const BUFFER_SIZE: usize = 1001;
const KEY_STRETCH: usize = 0;

fn main() {
    let mut kc = KeyChecker::new("abc");
    let mut key: usize = 0;

    for _ in 0..64 {
        key = kc.next_key();
     }

    println!("The 64th key was at index {}", key);
}
#[allow(dead_code)]
struct HashEntry {
    digest: String,
    three_runs: HashSet::<char>,
    five_runs: HashSet::<char>,
}

impl HashEntry {
    fn new(salt: &str, index: usize) -> Self {
        let mut data = format!("{}{}", salt, index);
        let mut digest = format!("{:x}", md5::compute(data));

        for _ in 0..KEY_STRETCH {
            data = digest;
            digest = format!("{:x}", md5::compute(data));
        }

        let mut three_runs = HashSet::new();
        let mut five_runs = HashSet::new();

        let chvec: Vec<_> = digest.chars().collect();
        for window in chvec[..].windows(3) {
            if window[0] == window[1] && window[1] == window[2] {
                three_runs.insert(window[0]);
                break;
            }
        }

        for window in chvec[..].windows(5) {
            if window[0] == window[1] && window[1] == window[2] && window[2] == window[3] && window[3] == window[4] {
                five_runs.insert(window[0]);
            }
        }

        Self {
            digest,
            three_runs,
            five_runs
        }
    }
}

struct KeyChecker {
    entries: [HashEntry; BUFFER_SIZE], // Pre-computed entries
    entry_idx: usize, // Where we are in the circular buffer

    salt: String, // Provided at initialization time
    key_idx: usize, // The index of the key that entry_idx currently points to
}

impl KeyChecker {
    fn new(salt: &str) -> Self {
        let entries: [HashEntry; BUFFER_SIZE] = core::array::from_fn(|i| HashEntry::new(salt, i));
        let entry_idx = 0; // Start off pointing to the beginning of the buffer
        let salt = salt.to_string();
        let key_idx = 0;

        Self { entries, entry_idx, salt, key_idx }
    }

    fn check(&self) -> bool {
        for c in self.entries[self.entry_idx].three_runs.iter() {
            for i in 0..BUFFER_SIZE {
                if i == self.entry_idx {
                    continue;
                }

                if self.entries[i].five_runs.contains(c) {
                    return true;
                }
            }
        }
        false
    }

    fn step(&mut self) {
        self.entries[self.entry_idx] = HashEntry::new(&self.salt, self.key_idx + BUFFER_SIZE);

        self.entry_idx = (self.entry_idx + 1) % BUFFER_SIZE;
        self.key_idx += 1;
    }

    fn next_key(&mut self) -> usize {
        self.step();
        while !self.check() {
            self.step();
        }
        self.key_idx
    }
}

#[test]
fn test_new_hashentry() {
    let entry = HashEntry::new("abc", 18);
    assert!(entry.three_runs.contains(&'8'));
}

#[test]
fn test_keychecker() {
    let mut kc = KeyChecker::new("abc");
    assert_eq!(kc.check(), false);
    assert_eq!(kc.next_key(), 39);
    assert_eq!(kc.next_key(), 92);
}