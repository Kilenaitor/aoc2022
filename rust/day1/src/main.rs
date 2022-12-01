use anyhow::Result;

fn main() -> Result<()> {
    let mut calories: Vec<u32> = include_str!("input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.split("\n").fold(0_u32, |total_calories, calories| {
                total_calories + calories.parse::<u32>().unwrap()
            })
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    println!("The highest elf has {} calores", calories[0]);

    println!(
        "The top three elves have {} total calories",
        calories[0] + calories[1] + calories[2]
    );

    Ok(())
}
