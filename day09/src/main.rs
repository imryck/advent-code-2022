use std::fmt::Error;

use common;

const ROPE_LENGTH: usize = 10;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

// TODO refactor with struct
type Position = (i32, i32);

fn main() {
    let cli_args = common::parse_cli_args();
    let lines = common::read_lines(&cli_args.input_path).unwrap();

    let mut knots: [Position; ROPE_LENGTH] = [(0, 0); ROPE_LENGTH];
    let mut point_visited: Vec<String> = Vec::new();
    let moves = parse_moves(lines);
    moves.into_iter().for_each(|(dir, dist)| {
        for _i in 0..dist {
            for j in 0..ROPE_LENGTH {
                if j == 0 {
                    knots[j] = move_knot(&dir, knots[j]);
                } else if !(is_near(knots[j], knots[j - 1])) {
                    knots[j] = follow_knot(knots[j], knots[j - 1]);
                }
            }
            register_point(knots[ROPE_LENGTH - 1], &mut point_visited);
        }
    });
    dbg!(knots);
    dbg!(point_visited.len());
}

fn is_near(pos_a: Position, pos_b: Position) -> bool {
    pos_a.0 - 1 <= pos_b.0 && pos_b.0 <= pos_a.0 + 1 &&
        pos_a.1 - 1 <= pos_b.1 && pos_b.1 <= pos_a.1 + 1
}

fn register_point(pos: Position, point_visited: &mut Vec<String>) {
    let point = format_point(pos);
    if !point_visited.contains(&point) {
        point_visited.push(point);
    }
}

fn format_point(pos: Position) -> String {
    format!("{}:{}", pos.0, pos.1)
}

fn move_knot(dir: &Direction, pos: Position) -> Position {
    let mut new_pos = pos.clone();
    match dir {
        Direction::Up => new_pos.1 += 1,
        Direction::Right => new_pos.0 += 1,
        Direction::Down => new_pos.1 -= 1,
        Direction::Left => new_pos.0 -= 1,
    };
    new_pos
}

fn follow_knot(pos: Position, ahead: Position) -> Position {
    let mut x: i32 = pos.0;
    let mut y: i32 = pos.1;
    if pos.0 != ahead.0 {
        x += (ahead.0 - pos.0) / (ahead.0 - pos.0).abs();
    }
    if y != ahead.1 {
        y += (ahead.1 - pos.1) / (ahead.1 - pos.1).abs();
    }
    (x, y)
}

fn parse_moves(lines: Vec<String>) -> Vec<(Direction, i32)> {
    lines.into_iter()
        .map(|l| l.split(" ").map(String::from).collect::<Vec<String>>())
        .map(|parts: Vec<String>| (match_direction(parts[0].clone()).unwrap(), parts[1].parse::<i32>().unwrap()))
        .collect::<Vec<(Direction, i32)>>()
}

fn match_direction(dir: String) -> Result<Direction, Error> {
    match dir.as_str() {
        "U" => Ok(Direction::Up),
        "R" => Ok(Direction::Right),
        "D" => Ok(Direction::Down),
        "L" => Ok(Direction::Left),
        _ => panic!("Unknown direction"),
    }
}
