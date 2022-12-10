pub fn process_part_1(input: &str) -> String {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let rows = trees.len();
    let columns = trees[0].len();

    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(r, tree_row)| {
            tree_row
                .iter()
                .enumerate()
                .map(|(c, _)| if r == 0 || r == rows - 1 || c == 0 || c == columns - 1 {true} else {false})
                .collect()
        })
        .collect();

    let mut current_tree_height = 0;

    // North
    (0..columns).for_each(|c| {
        current_tree_height = 0;

        (0..rows).for_each(|r| {
            if r == 0 {
                current_tree_height = trees[r][c];
            } else if trees[r][c] > current_tree_height {
                current_tree_height = trees[r][c];
                visible_trees[r][c] = true;
            }
        });
    });

    // West
    (0..rows).for_each(|r| {
        current_tree_height = 0;

        (0..columns).for_each(|c| {
            if c == 0 {
                current_tree_height = trees[r][c];
            } else if trees[r][c] > current_tree_height {
                current_tree_height = trees[r][c];
                visible_trees[r][c] = true;
            }
        });
    });

    // South
    (0..columns).rev().for_each(|c| {
        current_tree_height = 0;

        (0..rows).rev().for_each(|r| {
            if r == rows - 1 {
                current_tree_height = trees[r][c];
            } else if trees[r][c] > current_tree_height {
                current_tree_height = trees[r][c];
                visible_trees[r][c] = true;
            }
        });
    });

    // East
    (0..rows).rev().for_each(|r| {
        current_tree_height = 0;

        (0..columns).rev().for_each(|c| {
            if c == columns - 1 {
                current_tree_height = trees[r][c];
            } else if trees[r][c] > current_tree_height {
                current_tree_height = trees[r][c];
                visible_trees[r][c] = true;
            }
        });
    });

    let result: usize = visible_trees
        .iter()
        .flatten()
        .filter(|&&visible| visible)
        .count();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let rows = trees.len();
    let columns = trees[0].len();

    let mut result = 0;

    trees.iter().enumerate().for_each(|(r, tree_row)| {
        tree_row.iter().enumerate().for_each(|(c, &tree_height)| {
            let mut scores = [0, 0, 0, 0];

            // North
            for i in (0..r).rev() {
                scores[0] += 1;

                if tree_height <= trees[i][c] {break}
            }

            // West
            for i in (0..c).rev() {
                scores[1] += 1;

                if tree_height <= trees[r][i] {break}
            }

            // South
            for i in (r + 1)..rows {
                scores[2] += 1;

                if tree_height <= trees[i][c] {break}
            }

            // East
            for i in (c + 1)..columns {
                scores[3] += 1;

                if tree_height <= trees[r][i] {break}
            }

            let scenic_score = scores.iter().product();

            if scenic_score > result {
                result = scenic_score;
            }
        });
    });

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
