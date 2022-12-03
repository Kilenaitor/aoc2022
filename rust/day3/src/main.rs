use array_tool::vec::Intersect;

fn main() {
    part1();
    part2();
}

fn part2() {
    let rucksacks = include_str!("input.txt").split("\n");

    let mut groups = vec![];
    let mut group = vec![];
    for rucksack in rucksacks {
        group.push(rucksack.as_bytes().to_vec());
        if group.len() == 3 {
            groups.push(group);
            group = vec![];
        }
    }

    let priorities: u32 = groups.iter().map(|group| get_group_priority(group)).sum();
    println!("{}", priorities);
}

fn get_group_priority(group: &Vec<Vec<u8>>) -> u32 {
    let common = group[0].clone().intersect(group[1].clone());
    let shared_letter = common.intersect(group[2].clone());
    get_priority(shared_letter[0] as char)
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
    let (half1, half2) = split(&rucksack);
    let common_letter = half1.intersect(half2)[0];
    get_priority(common_letter as char)
}

fn split(rucksack: &str) -> (Vec<u8>, Vec<u8>) {
    let (half1, half2) = rucksack.split_at(rucksack.len() / 2);
    (half1.as_bytes().to_vec(), half2.as_bytes().to_vec())
}

fn get_priority(letter: char) -> u32 {
    let ascii_value = letter as u32;
    if ascii_value >= 65 && ascii_value <= 90 {
        ascii_value - 38
    } else if ascii_value >= 97 && ascii_value <= 122 {
        ascii_value - 96
    } else {
        panic!("Invalid ascii value")
    }
}
