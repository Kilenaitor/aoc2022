use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part2() {
    let priorities: u32 = include_str!("input.txt")
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| get_chunk_priority(chunk))
        .sum();
    println!("{}", priorities);
}

fn get_chunk_priority(group: &[&str]) -> u32 {
    let shared_letter: char = group
        .iter()
        .map(|rucksack| HashSet::from_iter(rucksack.chars()))
        .reduce(|a: HashSet<char>, b| a.intersection(&b).cloned().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    get_priority(&shared_letter)
}

fn part1() {
    let priorities: u32 = include_str!("input.txt")
        .split("\n")
        .filter(|rucksack| rucksack.len() > 0)
        .map(|rucksack| get_rucksack_priority(rucksack))
        .sum();
    println!("{}", priorities);
}

fn get_rucksack_priority(rucksack: &str) -> u32 {
    let (half1, half2) = rucksack.split_at(rucksack.len() / 2);
    let half1: HashSet<char> = HashSet::from_iter(half1.chars());
    let half2 = HashSet::from_iter(half2.chars());
    let shared_letter = half1.intersection(&half2).next().unwrap();
    get_priority(shared_letter)
}

fn get_priority(letter: &char) -> u32 {
    let ascii_value = *letter as u32;
    if ascii_value >= 65 && ascii_value <= 90 {
        ascii_value - 38
    } else if ascii_value >= 97 && ascii_value <= 122 {
        ascii_value - 96
    } else {
        panic!("Invalid ascii value")
    }
}
