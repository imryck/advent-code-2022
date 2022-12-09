// use std::fmt::Error;
//
// use common;
//
// #[derive(Debug, PartialEq)]
// enum Direction {
//     Up,
//     Right,
//     Down,
//     Left,
// }
//
// type Position = (i32, i32);
//
// fn main() {
//     let cli_args = common::parse_cli_args();
//     let lines = common::read_lines(&cli_args.input_path).unwrap();
//
//     let mut point_visited: Vec<String> = Vec::new();
//     let mut head_pos = (0, 0);
//     let mut tail_pos = (0, 0);
//     parse_moves(lines).into_iter().for_each(|(dir, dist)| {
//         head_pos = move_knot(&dir, dist, head_pos);
//         let new_tail_pos = get_new_tail_pos(tail_pos, head_pos, &dir);
//         // dbg!(("before", tail_pos));
//         // dbg!(("after", new_tail_pos));
//
//
//         // if dir == Direction::Up || dir == Direction::Down {
//         //     // Vertical
//         //
//         //     if tail_pos.0 != new_tail_pos.1 {}
//         //
//         // } else {
//         //     // Horizontal
//         // }
//
//         match dir {
//             Direction::Up => {
//                 let y_start = if tail_pos.0 != new_tail_pos.0 { tail_pos.1 + 1 } else { tail_pos.1 };
//                 for i in y_start..new_tail_pos.1 + 1 { register_point((new_tail_pos.0, i), &mut point_visited) }
//             }
//             Direction::Right => {
//                 let x_start = if tail_pos.1 != new_tail_pos.1 { tail_pos.0 + 1 } else { tail_pos.0 };
//                 for i in x_start..new_tail_pos.0 + 1 { register_point((i, new_tail_pos.1), &mut point_visited) }
//             },
//             Direction::Down => {
//                 let y_start = if tail_pos.0 != new_tail_pos.0 { tail_pos.1 - 1 } else { tail_pos.1 };
//                 for i in new_tail_pos.1..y_start + 1 { register_point((new_tail_pos.0, i), &mut point_visited) }
//             },
//             Direction::Left => {
//                 let x_start = if tail_pos.1 != new_tail_pos.1 { tail_pos.0 - 1 } else { tail_pos.0 };
//                 for i in new_tail_pos.0..x_start + 1 { register_point((i, new_tail_pos.1), &mut point_visited) }
//             },
//         };
//
//
//         tail_pos = new_tail_pos;
// //         dbg!(head_position);
// //         dbg!(tail_position);
//     });
//
//     dbg!(point_visited.len());
// }
//
// fn register_point(pos: Position, point_visited: &mut Vec<String>) {
//     let point = format_point(pos);
//     // println!("{point}");
//     if !point_visited.contains(&point) {
//         point_visited.push(point);
//     }
// }
//
// fn move_knot(dir: &Direction, dist: i32, pos: Position) -> Position {
//     let mut new_pos = pos.clone();
//     match dir {
//         Direction::Up => new_pos.1 += dist,
//         Direction::Right => new_pos.0 += dist,
//         Direction::Down => new_pos.1 -= dist,
//         Direction::Left => new_pos.0 -= dist,
//     };
//     new_pos
// }
//
// fn get_new_tail_pos(tail_pos: Position, new_head_pos: Position, dir: &Direction) -> Position {
//     let mut new_tail_pos = tail_pos.clone();
//     if tail_pos.0 - 1 <= new_head_pos.0 && new_head_pos.0 <= tail_pos.0 + 1 &&
//         tail_pos.1 - 1 <= new_head_pos.1 && new_head_pos.1 <= tail_pos.1 + 1 {
//         new_tail_pos
//     } else {
//         match dir {
//             Direction::Up => new_tail_pos = (new_head_pos.0, new_head_pos.1 - 1),
//             Direction::Right => new_tail_pos = (new_head_pos.0 - 1, new_head_pos.1),
//             Direction::Down => new_tail_pos = (new_head_pos.0, new_head_pos.1 + 1),
//             Direction::Left => new_tail_pos = (new_head_pos.0 + 1, new_head_pos.1),
//         };
//         new_tail_pos
//     }
// }
//
// fn format_point(pos: Position) -> String {
//     format!("{}:{}", pos.0, pos.1)
// }
//
// fn parse_moves(lines: Vec<String>) -> Vec<(Direction, i32)> {
//     lines.into_iter()
//         .map(|l| l.split(" ").map(String::from).collect::<Vec<String>>())
//         .map(|parts: Vec<String>| (match_direction(parts[0].clone()).unwrap(), parts[1].parse::<i32>().unwrap()))
//         .collect::<Vec<(Direction, i32)>>()
// }
//
// fn match_direction(dir: String) -> Result<Direction, Error> {
//     match dir.as_str() {
//         "U" => Ok(Direction::Up),
//         "R" => Ok(Direction::Right),
//         "D" => Ok(Direction::Down),
//         "L" => Ok(Direction::Left),
//         _ => panic!("Unknown direction"),
//     }
// }
