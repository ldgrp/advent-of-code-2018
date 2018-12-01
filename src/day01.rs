use std::collections::HashSet;
use std::io::{self, BufRead};
use std::vec::Vec;

pub fn solve() {
    let vec = read_input();
    println!("Part One: {}", solve_one(&vec));
    println!("Part Two: {}", solve_two(&vec));
}

fn solve_one(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

fn solve_two(vec: &[i32]) -> i32 {
    let mut set_sums = HashSet::new();
    set_sums.insert(0);
    let mut sum = 0;

    let mut elements = vec.iter().cycle();
    loop {
        sum += elements.next().unwrap();
        if !set_sums.contains(&sum) {
            set_sums.insert(sum);
        } else {
            break
        }
    }

    sum
}

fn read_input() -> Vec<i32> {
    let mut vec = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        vec.push(line.unwrap().parse::<i32>().unwrap());
    }

    vec
}
