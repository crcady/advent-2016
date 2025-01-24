use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let numvecs: Vec<_> = lines.iter().map(|x| parse_line(x)).collect();
    let triangles: Vec<_> = numvecs.iter().filter(|x|test_nums(x)).collect();

    let ascols = to_cols(&numvecs);
    let coltriangles: Vec<_> = ascols.iter().filter(|x|test_nums(x)).collect();

    println!("Found {} triangles", triangles.len());
    println!("Found {} triangles from columns", coltriangles.len());
}

fn parse_line(line: &str) -> Vec<u32> {
    line.trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .map(|x| x.unwrap())
        .collect()
}

fn test_nums(nums: &Vec<u32>) -> bool {
    let mut sorted_nums: Vec<u32> = nums.to_vec();
    sorted_nums.sort();
    sorted_nums[0] + sorted_nums[1] > sorted_nums[2]
}

fn to_cols(numvecs: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let all_the_numbers: Vec<_> = numvecs.iter().flatten().copied().collect();
    let column_one: Vec<_> = all_the_numbers.iter().step_by(3).copied().collect();
    let column_two: Vec<_> = all_the_numbers.iter().skip(1).step_by(3).copied().collect();
    let column_three: Vec<_> = all_the_numbers.iter().skip(2).step_by(3).copied().collect();
    let all_the_numbers: Vec<_> = [&column_one[..], &column_two[..], &column_three[..]].concat();
    let mut res: Vec<Vec<u32>> = Vec::new();
    let mut n: usize = 0;
    while n < all_the_numbers.len() {
        res.push(vec![all_the_numbers[n], all_the_numbers[n+1], all_the_numbers[n+2]]);
        n += 3;
    }
    res
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("5 10 25"), vec![5, 10, 25]);
}

#[test]
fn test_test_nums() {
    assert_eq!(test_nums(&vec![5, 10, 25]), false);
    assert_eq!(test_nums(&vec![5, 3, 4]), true);
}

#[test]
fn test_to_cols(){
    let rows: Vec<Vec<u32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let cols: Vec<Vec<u32>> = vec![
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![3, 6, 9],
    ];

    assert_eq!(to_cols(&rows), cols);
}