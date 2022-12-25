use std::collections::BTreeSet;

use itertools::Itertools;

use nom::*;
use nom::bytes::streaming::tag;
use nom::character::complete::{self, line_ending};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn rock_path(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, rock_path) = separated_list1(tag(" -> "), separated_pair(complete::u32, tag(","), complete::u32))(input)?;

    let rocks = rock_path
        .into_iter()
        .tuple_windows()
        .flat_map(|((ax, ay), (bx, by))| {
            let x_min = ax.min(bx);
            let x_max = ax.max(bx);
            let x_range = x_min..=x_max;

            let y_min = ay.min(by);
            let y_max = ay.max(by);
            let y_range = y_min..=y_max;

            x_range.cartesian_product(y_range)
        })
        .collect();

    Ok((input, rocks))
}

fn rocks(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, rocks) = separated_list1(line_ending, rock_path)(input)?;

    let rocks = rocks
        .into_iter()
        .flatten()
        .collect();

    Ok((input, rocks))
}

pub fn process_part_1(input: &str) -> String {
    let mut rocks = rocks(input).unwrap().1;

    let rock_count = rocks.len();

    let mut rocks_vec: Vec<(u32, u32)> = rocks
        .clone()
        .into_iter()
        .collect();

    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let lowest_rock = rocks_vec
        .last()
        .unwrap();

    let mut sand = (500, 0);

    loop {
        if sand.1 > lowest_rock.1 {break}

        let down = (sand.0, sand.1 + 1);
        let left = (sand.0 - 1, sand.1 + 1);
        let right = (sand.0 + 1, sand.1 + 1);

        let down_check = rocks.get(&down);
        let left_check = rocks.get(&left);
        let right_check = rocks.get(&right);

        match (down_check, left_check, right_check) {
            (None, _, _) => sand = down,
            (_, None, _) => sand = left,
            (_, _, None) => sand = right,
            (Some(_), Some(_), Some(_)) => {
                rocks.insert(sand);

                sand = (500, 0)
            },
        }
    }

    let result = rocks.len() - rock_count;

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let mut rocks = rocks(input).unwrap().1;

    let rock_count = rocks.len();

    let mut rocks_vec: Vec<(u32, u32)> = rocks
        .clone()
        .into_iter()
        .collect();

    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let lowest_rock = rocks_vec
        .last()
        .unwrap();

    let floor = lowest_rock.1 + 2;

    let mut sand = (500, 0);

    while let None = rocks.get(&(500, 0)) {
        let down = (sand.0, sand.1 + 1);
        let left = (sand.0 - 1, sand.1 + 1);
        let right = (sand.0 + 1, sand.1 + 1);

        let down_check = rocks
            .get(&down)
            .or_else(|| {
                if down.1 == floor {Some(&lowest_rock)} else {None}
            });

        let left_check = rocks
            .get(&left)
            .or_else(|| {
                if left.1 == floor {Some(&lowest_rock)} else {None}
            });

        let right_check = rocks
            .get(&right)
            .or_else(|| {
                if right.1 == floor {Some(&lowest_rock)} else {None}
            });

        match (down_check, left_check, right_check) {
            (Some(_), Some(_), Some(_)) => {
                rocks.insert(sand);

                sand = (500, 0)
            },
            (None, _, _) => sand = down,
            (_, None, _) => sand = left,
            (_, _, None) => sand = right,
        }
    }

    let result = rocks.len() - rock_count;

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT);

        assert_eq!(result, "24");
    }

    #[test]
    fn test_part_2() {
        let result = process_part_2(INPUT);

        assert_eq!(result, "93");
    }
}

