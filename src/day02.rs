use std::collections::HashMap;
use std::io::{self, BufRead};
use std::vec::Vec;

pub fn solve() {
    let vec = read_input();
    println!("Part One: {}", solve_one(&vec));
    println!("Part Two: {:?}", solve_two(&vec).unwrap());
}

fn solve_one(words: &[String]) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for word in words {
        let mut char_count = HashMap::new();

        for ch in word.chars() {
            let cnt = char_count.entry(ch).or_insert(0);
            *cnt += 1;
        }
        
        if char_count.values().any(|&x| x == 2) {
            twos += 1;
        }
        if char_count.values().any(|&x| x == 3) {
            threes += 1;
        }
    }

    twos * threes
}

fn solve_two(words: &[String]) -> Option<String> {

    for i in 0..words.len() {
        for j in i+1..words.len() {
            let common_letters = find_common_letters(&words[i], &words[j]);
            if words[i].len() - common_letters.len() == 1 {
                return Some(common_letters)
            }
        }
    }
    None
}

fn find_common_letters(word1: &String, word2: &String) -> String {
    let mut common_letters = String::new();

    for (c1, c2) in word1.chars().zip(word2.chars()) {
        if c1 == c2 {
            common_letters.push(c1);
        }
    }
    common_letters
}

fn read_input() -> Vec<String> {
    let mut vec = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        vec.push(line.unwrap());
    }

    vec
}
