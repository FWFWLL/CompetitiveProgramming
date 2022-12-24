use itertools::Itertools;

use nom::*;
use nom::bytes::streaming::tag;
use nom::character::complete::{newline, self};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

#[derive(Debug)]
enum Material {
    Air,
    Rock,
    Sand,
}

fn rock_path(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, rock_path) = separated_list1(tag(" -> "), separated_pair(complete::u32, tag(","), complete::u32))(input)?;

    Ok((input, rock_path))
}

fn parse_rock_paths(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    let (input, rock_paths) = separated_list1(newline, rock_path)(input)?;

    Ok((input, rock_paths))
}

pub fn process_part_1(input: &str) -> String {
    let rock_paths = parse_rock_paths(input).unwrap().1;

    dbg!(&rock_paths);

    let (min_x, min_y, max_x, max_y) = get_bounds(&rock_paths);

    let offset_x = max_x - min_x;
    let offset_y = max_y - min_y;

    let materials: Vec<Vec<Material>> = populate_material_vec(rock_paths, offset_x, offset_y);

    dbg!(&materials);

    let display: String = materials
        .iter()
        .map(|vec| {
            vec
                .iter()
                .map(|material| {
                    match material {
                        Material::Air => ".",
                        Material::Rock => "#",
                        Material::Sand => "o",
                    }
                }).collect()
        })
        .intersperse("\n".to_string())
        .collect();

    println!("{display}");

    todo!("Part 1");
}

fn populate_material_vec(rock_paths: Vec<Vec<(u32, u32)>>, offset_x: u32, offset_y: u32) -> Vec<Vec<Material>> {
    let mut materials: Vec<Vec<Material>> = vec![];

    // Fill with air
    (0..=offset_y as usize).for_each(|y| {
        materials.push(vec![]);

        (0..=offset_x).for_each(|_| {
            materials[y].push(Material::Air);
        });
    });

    return materials;
}

// Probably a better way to do this
fn get_bounds(rock_paths: &Vec<Vec<(u32, u32)>>) -> (u32, u32, u32, u32) {
    let min_x = *rock_paths
        .iter()
        .flat_map(|path| path.iter().map(|(x, _)| x))
        .min()
        .unwrap();

    dbg!(&min_x);

    let max_x = *rock_paths
        .iter()
        .flat_map(|path| path.iter().map(|(x, _)| x))
        .max()
        .unwrap();

    dbg!(&max_x);

    let min_y = 0;

    dbg!(&min_y);

    let max_y = *rock_paths
        .iter()
        .flat_map(|path| path.iter().map(|(_, y)| y))
        .max()
        .unwrap();

    dbg!(&max_y);

    (min_x, min_y, max_x, max_y)
}

pub fn process_part_2(input: &str) -> String {
    todo!("Part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT);

        assert_eq!(result, "");
    }

    #[test]
    fn test_part_2() {
        let result = process_part_2(INPUT);

        assert_eq!(result, "");
    }
}

