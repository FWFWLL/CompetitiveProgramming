use std::collections::BTreeSet;

use itertools::Itertools;
use lending_iterator::prelude::*;
use nom::{*, character::complete::{self, newline}, branch::alt, multi::separated_list1, sequence::separated_pair, bytes::complete::tag};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('U').map(|_| Direction::Up),
        complete::char('L').map(|_| Direction::Left),
        complete::char('D').map(|_| Direction::Down),
    ))(input)?;

    Ok((input, direction))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, moves) = separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;

    let moves = moves
        .iter()
        .flat_map(|(direction, repeat)| vec![*direction; *repeat as usize])
        .collect();

    Ok((input, moves))
}

pub fn process_part_1(input: &str) -> String {
    let moves = parse_moves(input).unwrap().1;

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = BTreeSet::from([tail]);

    moves.iter().for_each(|head_move| {
        match head_move {
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 += 1,
            Direction::Left => head.0 -= 1,
            Direction::Down => head.1 -= 1,
        }
        
        let x_range = (head.0 - 1)..=(head.0 + 1);
        let y_range = (head.1 - 1)..=(head.1 + 1);

        let tail_is_connected = x_range
            .cartesian_product(y_range)
            .any(|pos| pos == tail);

        if !tail_is_connected {
            let mut new_tail = head.clone();

            match head_move {
                Direction::Right => new_tail.0 -= 1,
                Direction::Up => new_tail.1 -= 1,
                Direction::Left => new_tail.0 += 1,
                Direction::Down => new_tail.1 += 1,
            }

            tail = new_tail;
            visited.insert(new_tail);
        }
    });

    let result = visited.len();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let moves = parse_moves(input).unwrap().1;

    let mut rope = [(0, 0); 10];

    let mut visited = BTreeSet::from([*rope.last().unwrap()]);

    moves.iter().for_each(|head_move| {
        match head_move {
            Direction::Right => rope[0].0 += 1,
            Direction::Up => rope[0].1 += 1,
            Direction::Left => rope[0].0 -= 1,
            Direction::Down => rope[0].1 -= 1,
        }

        let mut rope_segments = rope.windows_mut::<2>();

        while let Some([ref mut head, ref mut tail]) = rope_segments.next() {
            let x_range = (head.0 - 1)..=(head.0 + 1);
            let y_range = (head.1 - 1)..=(head.1 + 1);

            let tail_is_connected = x_range
                .cartesian_product(y_range)
                .any(|pos| pos == *tail);

            if !tail_is_connected {
                if head.0 == tail.0 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                } else {
                    let x_range = (head.0 - 1)..=(head.0 + 1);
                    let y_range = (head.1 - 1)..=(head.1 + 1);

                    let head_3x3: Vec<(i32, i32)> = x_range
                        .cartesian_product(y_range)
                        .collect();

                    let x_range = (tail.0 - 1)..=(tail.0 + 1);
                    let y_range = (tail.1 - 1)..=(tail.1 + 1);

                    let possible_new_tails: Vec<(i32, i32)> = x_range
                        .cartesian_product(y_range)
                        .filter(|pos| head_3x3.contains(pos))
                        .collect();

                    match possible_new_tails.len() {
                        2 => {
                            let new_head_cross_position = [
                                (head.0 + 1, head.1),
                                (head.0 - 1, head.1),
                                (head.0, head.1 + 1),
                                (head.0, head.1 - 1),
                            ];

                            let next = possible_new_tails
                                .iter()
                                .find(|pos| new_head_cross_position.contains(pos))
                                .unwrap();

                            *tail = *next;
                        },
                        1 => *tail = possible_new_tails[0],
                        _ => panic!("Unknown tail length"),
                    };
                }
            }
        }

        visited.insert(*rope.last().unwrap());
    });

    let result = visited.len();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./test_input_1.txt").unwrap();

        let result = process_part_1(&input);

        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_2() {
        let input_1 = fs::read_to_string("./test_input_1.txt").unwrap();
        let input_2 = fs::read_to_string("./test_input_2.txt").unwrap();

        let result_1 = process_part_2(&input_1);
        let result_2 = process_part_2(&input_2);

        assert_eq!(result_1, "1");
        assert_eq!(result_2, "36");
    }
}
