pub fn process_part_1(input: &str) -> String {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;
    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            let curr_tree_height = map[i][j];

            // North
            let mut not_visible_n = false;
            for k in (0..i).rev() {
                if curr_tree_height <= map[k][j] {
                    not_visible_n = true;
                    break;
                }
            }

            if not_visible_n == false {
                sum += 1;
                continue;
            }

            // West
            let mut not_visible_w = false;
            for k in (0..j).rev() {
                if curr_tree_height <= map[i][k] {
                    not_visible_w = true;
                    break;
                }
            }

            if not_visible_w == false {
                sum += 1;
                continue;
            }

            // South
            let mut not_visible_s = false;
            for k in (i + 1)..map.len() {
                if curr_tree_height <= map[k][j] {
                    not_visible_s = true;
                    break;
                }
            }

            if not_visible_s == false {
                sum += 1;
                continue;
            }

            // East
            let mut not_visible_e = false;
            for k in (j + 1)..map[i].len() {
                if curr_tree_height <= map[i][k] {
                    not_visible_e = true;
                    break;
                }
            }

            if not_visible_e == false {
                sum += 1;
                continue;
            }
        }
    }

    let result = sum + (2 * map.len()) + (2 * map[0].len()) - 4;

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut scenic_scores: Vec<u32> = Vec::new();

    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            let curr_tree_height = map[i][j];

            // North
            let mut visible_trees_n = 0;
            for k in (0..i).rev() {
                visible_trees_n += 1;
                if curr_tree_height <= map[k][j] {
                    break;
                }
            }

            // West
            let mut visible_trees_w = 0;
            for k in (0..j).rev() {
                visible_trees_w += 1;
                if curr_tree_height <= map[i][k] {
                    break;
                }
            }

            // South
            let mut visible_trees_s = 0;
            for k in (i + 1)..map.len() {
                visible_trees_s += 1;
                if curr_tree_height <= map[k][j] {
                    break;
                }
            }

            // East
            let mut visible_trees_e = 0;
            for k in (j + 1)..map[i].len() {
                visible_trees_e += 1;
                if curr_tree_height <= map[i][k] {
                    break;
                }
            }

            scenic_scores.push(visible_trees_n * visible_trees_w * visible_trees_e * visible_trees_s)
        }
    }

    let result = scenic_scores.iter().max().unwrap();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./test_input.txt").unwrap();

        let result = process_part_1(&input);

        assert_eq!(result, "21");
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./test_input.txt").unwrap();

        let result = process_part_2(&input);

        assert_eq!(result, "8");
    }
}
