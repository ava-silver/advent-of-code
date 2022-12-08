use std::fs::read_to_string;
fn parse_input(input: String) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect()
}


fn is_visible(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> bool {
    // its visible if the trees in the column and row are less than it
    let h = trees[y][x];
    let left = *trees[y][..x].iter().max().unwrap();
    let right =  *trees[y][x+1..].iter().max().unwrap();
    let above = trees[..y].iter().map(|r| r[x]).max().unwrap();
    let below = trees[y+1..].iter().map(|r| r[x]).max().unwrap();
    *[left, right, above, below].iter().min().unwrap() < h
}

fn part_1(trees: Vec<Vec<u8>>) -> usize {
    let height = trees.len();
    let width = trees[0].len();
    let mut visible_trees = (height * 2) + (width - 2) * 2;
    for y in 1..height-1 {
        for x in 1..width-1 {
            if is_visible(x,y, &trees) {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn viewing_distance(others: Vec<u8>, val: u8) -> u64 {
    let s = others.len() as u64;
    others.into_iter().enumerate().find_map(|(idx, tree)| {
        if tree >= val {
            return Some((idx + 1) as u64);
        }
        None
    }).unwrap_or(s )
}

fn scenic_score(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> u64 {
    let h = trees[y][x];
    let left = viewing_distance(trees[y][..x].iter().map(|t| *t).rev().collect(), h);
    let right = viewing_distance( trees[y][x+1..].iter().map(|t| *t).collect(), h);
    let above = viewing_distance(trees[..y].iter().map(|r| r[x]).rev().collect(), h);
    let below = viewing_distance(trees[y+1..].iter().map(|r| r[x]).collect(), h);
    println!("{} {} {} {}", left, right, above, below);
    left * right * above * below
}

fn part_2(trees: Vec<Vec<u8>>) -> u64 {
    let height = trees.len();
    let width = trees[0].len();
    let mut max_scenic_score = 0;
    for y in 1..height-1 {
        for x in 1..width-1 {
            print!("{} {}, h: {}, ", x, y, trees[y][x]);
            let score = scenic_score(x,y, &trees);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }
    max_scenic_score
}



fn main() {
    let input = read_to_string("input.txt").unwrap();
    let trees = parse_input(input);
    println!("Part 1: {:?}", part_1(trees.clone()));
    println!("Part 2: {:?}", part_2(trees));
}
