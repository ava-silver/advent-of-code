use std::{fs::read_to_string, collections::HashSet};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Move {
    L,
    R,
    U,
    D,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos(i64, i64);

#[allow(dead_code)]
fn display(segs: &Vec<Pos>, indices: bool) -> String {
    let min_x = segs.iter().map(|p| p.0).min().unwrap();
    let min_y = segs.iter().map(|p| p.1).min().unwrap();
    let max_x = segs.iter().map(|p| p.0).max().unwrap();
    let max_y = segs.iter().map(|p| p.1).max().unwrap();

    (min_y..=max_y).map(|y| {
        (min_x..=max_x).map(|x| match segs.iter().enumerate().find(|(_, p)| **p == Pos(x,y)) {
            Some((idx, _)) if indices => idx.to_string(),
            Some(_) => "#".to_string(),
            None => ".".to_string(),
        }).collect::<String>() + "\n"
    }).rev().collect()
}

fn parse_input(input: String) -> Vec<(Move, usize)> {
    input.lines().map(|l| {
        let mut tok = l.split(' ');
        match (tok.next(), tok.next().and_then(|v| v.parse().ok())) {
            (Some("L"), Some(v)) => (Move::L, v),
            (Some("R"), Some(v)) => (Move::R, v),
            (Some("U"), Some(v)) => (Move::U, v),
            (Some("D"), Some(v)) => (Move::D, v),
            _ => panic!("unexpected tokens"),
        }
    }).collect()
    
}

fn in_range(head: Pos, tail: Pos) -> bool {
    head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1
}


fn move_head(mv: Move, pos: Pos) -> Pos {
    match mv {
        Move::L => Pos(pos.0 - 1, pos.1),
        Move::R => Pos(pos.0 + 1, pos.1),
        Move::U => Pos(pos.0, pos.1 + 1),
        Move::D => Pos(pos.0, pos.1 - 1),
    }
}

fn dist(p1: Pos, p2: Pos) -> f32 {
    f32::sqrt(((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f32)
}

fn move_tail(head: Pos, tail: Pos) -> Pos {

    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;
    if x_diff.abs() == 2 && y_diff.abs() == 2  {
        // weird diagonal case
        Pos(head.0 + if x_diff.is_positive() {-1} else {1},
            head.1 + if y_diff.is_positive() {-1} else {1})
    } else {
        [Pos(head.0 + 1, head.1),
        Pos(head.0 - 1, head.1),
        Pos(head.0, head.1 - 1),
        Pos(head.0, head.1 + 1)].into_iter()
        .map(|p| (p, dist(tail, p)))
        .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
        .unwrap().0
    }

}


fn part_1(moves: Vec<(Move, usize)>) -> usize {
    let mut head = Pos(0, 0);
    let mut tail = Pos(0, 0);
    let mut tail_locations = HashSet::from([Pos(0,0)]);

    for (mv, dist) in moves {
        for _ in 0..dist {
            head = move_head(mv, head);
            if !in_range(head, tail) {
                tail = move_tail(head, tail);
                tail_locations.insert(tail);
            }
        }
    }
    tail_locations.len()
}

fn part_2(moves: Vec<(Move, usize)>, debug: usize) -> usize {
    let mut segs = vec![Pos(0, 0); 10];
    let mut tail_locations = HashSet::from([Pos(0,0)]);
    
    for (mv, dist) in moves {
        for _ in 0..dist {
            segs[0] = move_head(mv, segs[0]);
            for i in 1..10 {
                if !in_range(segs[i-1], segs[i]) {
                    segs[i] = move_tail(segs[i-1], segs[i]);
                }
            }
            tail_locations.insert(*segs.last().unwrap());
        }
        if debug > 1 {
            println!("{}", display(&segs, true));
        }
    }
    if debug > 0 {
        let v = tail_locations.clone().into_iter().collect();
        println!("{}", display(&v, false));
    }
    tail_locations.len()
}



fn main() {
    let input = read_to_string("input.txt").unwrap();
    let moves = parse_input(input);
    println!("Part 1: {}", part_1(moves.clone()));
    println!("Part 2: {}", part_2(moves, 0));

}
