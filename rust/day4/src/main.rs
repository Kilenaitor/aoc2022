use std::ops::Range;
fn main() {
    part1();
    part2();
}

fn part1() {
    let num_overlaps: u32 = include_str!("input.txt")
        .split("\n")
        .map(|pairs| {
            let ((low1, high1), (low2, high2)) = parsed_pairs(pairs);
            if low1 >= low2 && high1 <= high2 {
                return 1;
            }
            if low1 <= low2 && high1 >= high2 {
                return 1;
            }
            0
        })
        .sum();
    println!("{}", num_overlaps);
}

fn part2() {
    let num_overlaps: u32 = include_str!("input.txt")
        .split("\n")
        .map(|pairs| {
            let ((low1, high1), (low2, high2)) = parsed_pairs(pairs);
            let range1 = Range {
                start: low1,
                end: high1 + 1,
            };
            let range2 = Range {
                start: low2,
                end: high2 + 1,
            };
            if range1.contains(&range2.start) || range1.contains(&(range2.end - 1)) {
                return 1;
            }
            if range2.contains(&range1.start) || range2.contains(&(range1.end - 1)) {
                return 1;
            }
            return 0;
        })
        .sum();
    println!("{}", num_overlaps);
}

fn parsed_pairs(pairs: &str) -> ((u32, u32), (u32, u32)) {
    let (pair1, pair2) = pairs.split_once(",").unwrap();
    let (low1, high1) = pair1.split_once("-").unwrap();
    let (low2, high2) = pair2.split_once("-").unwrap();
    (
        (low1.parse::<u32>().unwrap(), high1.parse::<u32>().unwrap()),
        (low2.parse::<u32>().unwrap(), high2.parse::<u32>().unwrap()),
    )
}
