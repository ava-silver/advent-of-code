use std::fs::read_to_string;

fn unique(slice: &[(usize, char)], window_size: usize) -> bool {
    for x in 0..window_size {
        for y in x + 1..window_size {
            if slice[x].1 == slice[y].1 {
                return false;
            }
        }
    }
    true
}

fn marker(input: String, window_size: usize) -> usize {
    let chars = input.chars().enumerate().collect::<Vec<_>>();
    for window in chars.windows(window_size) {
        if unique(window, window_size) {
            return window[window_size - 1].0 + 1;
        }
    }
    return 0;
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", marker(input.clone(), 4));
    println!("Part 2: {}", marker(input.clone(), 14));
}
