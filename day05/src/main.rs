use std::collections::HashSet;

fn main() {
    let door_id = "abc";
    let mut password = String::new();
    let mut next_index = 0u64;

    for _ in 0..8 {
        let res = get_next_digit(&door_id, next_index);
        password.push(res.0);
        next_index = res.1 + 1;
    }
    println!("{}", &password);

    password = String::from("________");
    println!("{}", &password);
    next_index = 0;
    let mut found_digits: HashSet<usize> = HashSet::new();

    while found_digits.len() < 8 {
        let (next_char, maybe_position, last_index) = get_next_digit2(&door_id, next_index);
        if let Some(pos) = maybe_position {
            if !found_digits.contains(&pos) {
                password.replace_range(pos..pos + 1, &next_char.to_string());
                println!("{}", &password);
                found_digits.insert(pos);
            }
        }
        next_index = last_index + 1;
    }
}

fn compute_md5(s: &String) -> String {
    let digest = md5::compute(s);
    format!("{:x}", digest)
}

type DigitResult = (char, u64);
type DigitResult2 = (char, Option<usize>, u64);

fn get_next_digit(door_id: &str, start_index: u64) -> DigitResult {
    let mut index = start_index;
    'outer: loop {
        let digest: Vec<_> = compute_md5(&[door_id, &index.to_string()].concat())
            .chars()
            .collect();
        for i in 0..5 {
            if digest[i] != '0' {
                index += 1;
                continue 'outer;
            }
        }
        return (digest[5], index);
    }
}

fn get_next_digit2(door_id: &str, start_index: u64) -> DigitResult2 {
    let mut index = start_index;
    'outer: loop {
        let digest: Vec<_> = compute_md5(&[door_id, &index.to_string()].concat())
            .chars()
            .collect();
        for i in 0..5 {
            if digest[i] != '0' {
                index += 1;
                continue 'outer;
            }
        }
        if digest[5].is_ascii_digit() && digest[5] as u32 <= '7' as u32 {
            return (
                digest[6],
                Some((digest[5] as u32 - '0' as u32) as usize),
                index,
            );
        } else {
            return (digest[6], None, index);
        }
    }
}

#[test]
fn test_md5_compute() {
    let input = String::from("abc3231929");
    let hash = compute_md5(&input);
    let hash_as_bytes: Vec<_> = hash.chars().collect();
    for i in 0..5 {
        assert_eq!(hash_as_bytes[i], '0');
    }
    assert_eq!(hash_as_bytes[5], '1')
}

#[test]
fn test_get_next_digit() {
    let next_digit = get_next_digit("abc", 0);
    assert_eq!(next_digit.0, '1');
    assert_eq!(next_digit.1, 3231929);
}

#[test]
fn test_get_next_digit2() {
    let next_digit = get_next_digit2("abc", 0);
    assert_eq!(next_digit.0, '5');
    assert_eq!(next_digit.1, Some(1));
    assert_eq!(next_digit.2, 3231929);
}
