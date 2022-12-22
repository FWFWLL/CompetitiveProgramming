use std::iter;

use itertools::Itertools;

use nom::*;
use nom::character::complete::{newline, alpha1};
use nom::multi::separated_list1;

use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn height_map(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(newline, alpha1.map(|line: &str| line.chars().collect()))(input)
}

pub fn process_part_1(input: &str) -> String {
    let height_map = height_map(input).unwrap().1;

    let start = height_map
        .iter()
        .enumerate()
        .flat_map(|(i, char)| {
            char.iter()
                .enumerate()
                .zip(iter::repeat(i))
        })
        .find_map(|((x, &c), y)| if c == 'S' {Some((x as i32, y as i32))} else {None})
        .unwrap();

    let end = height_map
        .iter()
        .enumerate()
        .flat_map(|(i, char)| {
            char.iter()
                .enumerate()
                .zip(iter::repeat(i))
        })
        .find_map(|((x, &c), y)| if c == 'E' {Some((x as i32, y as i32))} else {None})
        .unwrap();

    let height_map: Vec<Vec<char>> = height_map
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| {
                    match c {
                        'S' => 'a',
                        'E' => 'z',
                        v => *v,
                    }
                })
                .collect()
        }).collect();

    let edges = (0i32..height_map.len() as i32)
        .cartesian_product(0i32..height_map[0].len() as i32)
        .flat_map(|(y, x)| {
            let current_node = (x, y);

            let possible_neighbours = vec![
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
            ];

            possible_neighbours
                .iter()
                .filter_map(|cell| {
                    height_map
                        .get(cell.1 as usize)
                        .and_then(|vec| vec.get(cell.0 as usize))
                        .and_then(|valid_cell| {
                            let current_node_height = height_map[y as usize][x as usize];

                            if current_node_height as u8 + 1 >= *valid_cell as u8 {Some((current_node, *cell))} else {None}
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    let graph = DiGraphMap::<(i32, i32), ()>::from_edges(&edges);

    let paths = dijkstra(&graph, start, Some(end), |_| 1);

    let result = paths[&(end.0, end.1)];

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let height_map = height_map(input).unwrap().1;

    let end = height_map
        .iter()
        .enumerate()
        .flat_map(|(i, char)| {
            char.iter()
                .enumerate()
                .zip(iter::repeat(i))
        })
        .find_map(|((x, &c), y)| if c == 'E' {Some((x as i32, y as i32))} else {None})
        .unwrap();

    let height_map: Vec<Vec<char>> = height_map
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| {
                    match c {
                        'S' => 'a',
                        'E' => 'z',
                        v => *v,
                    }
                })
                .collect()
        }).collect();

    let edges = (0i32..height_map.len() as i32)
        .cartesian_product(0i32..height_map[0].len() as i32)
        .flat_map(|(y, x)| {
            let current_node = (x, y);

            let possible_neighbours = vec![
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
            ];

            possible_neighbours
                .iter()
                .filter_map(|cell| {
                    height_map
                        .get(cell.1 as usize)
                        .and_then(|vec| vec.get(cell.0 as usize))
                        .and_then(|valid_cell| {
                            let current_node_height = height_map[y as usize][x as usize];

                            if current_node_height as u8 + 1 >= *valid_cell as u8 {Some((current_node, *cell))} else {None}
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    let graph = DiGraphMap::<(i32, i32), ()>::from_edges(
        edges.iter().map(|(a, b)| (*b, *a))
    );

    let paths = dijkstra(&graph, end, None, |_| 1);

    let result = paths
        .iter()
        .filter_map(|((x, y), steps)| if height_map[*y as usize][*x as usize] == 'a' {Some(steps)} else {None})
        .min()
        .unwrap();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT);

        assert_eq!(result, "31");
    }

    #[test]
    fn test_part_2() {
        let result = process_part_2(INPUT);

        assert_eq!(result, "29");
    }
}

