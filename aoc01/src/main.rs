use std::fs::read_to_string;


fn elf_counts(input: impl Into<String>) -> Vec<u32> {
    input.into().split("\n\n").map(|elf| elf.lines().map(|item| item.parse::<u32>().expect("Each item should be a valid u32")).sum()).collect()
}

fn max_calories(counts: &Vec<u32>) -> u32 {
    counts.to_owned().into_iter().max().expect("There should be at least one elf")
}

fn top_3_calories(counts: &Vec<u32>) -> u32 {
    let mut c = counts.to_owned();
    c.sort_by(|a, b| b.cmp(a));
    c.into_iter().take(3).sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let counts = elf_counts(&input);

    // Part A
    println!("max calories: {}", max_calories(&counts));
    // Part B
    println!("top 3 calories: {}", top_3_calories(&counts));
}
