use std::collections::HashMap;

pub fn process_part_1(input: &str) -> String {
    let priorities: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    let result: usize = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            first
                .chars()
                .find(|c| second.contains(*c))
                .unwrap()
        })
        .map(|c| *priorities.get(&c).unwrap())
        .sum();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let priorities: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    let result: usize = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            chunk[0]
                .chars()
                .find(|char| chunk[1].contains(*char) && chunk[2].contains(*char))
                .unwrap()

        })
        .map(|c| *priorities.get(&c).unwrap())
        .sum();

    return result.to_string();
}
