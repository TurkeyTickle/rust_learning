use std::{collections::HashMap, str::Lines};

static INPUT: &'static str = include_str!("input.txt");

pub fn get_dup_item_priority_sums() -> u32 {
    let lines: Lines = INPUT.lines();
    let letters = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut priority_map: HashMap<char, u8> = Default::default();
    let mut dups: Vec<u32> = Vec::new();

    let mut index: u8 = 1;
    for letter in letters.chars() {
        priority_map.insert(letter, index);
        index += 1;
    }

    for line in lines {
        let left = &line[..(line.len() / 2)];
        let right = &line[left.len()..];

        'outer: for a in left.chars() {
            for b in right.chars() {
                if a == b {
                    dups.push(u32::from(*priority_map.get(&a).unwrap()));
                    break 'outer;
                }
            }
        };
    }

    dups.iter().sum()
}