use std::collections::HashSet;

pub fn process_part_1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| get_common_char(first, second))
        .map(|c| get_priority(c))
        .sum();

    return result.to_string();
}

fn get_common_char(a: &str, b: &str) -> char {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    set_a
        .into_iter()
        .filter(|c| set_b.contains(&c))
        .next()
        .unwrap()
}

fn get_priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

pub fn process_part_2(input: &str) -> String {
    let mut lines = input.lines();

    let mut sum = 0;
    loop {
        if lines.clone().peekable().peek().is_none() {break;}

        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        sum += get_priority(get_common_tri_char(a, b, c));
    }

    return sum.to_string();
}

fn get_common_tri_char(a: &str, b: &str, c: &str) -> char {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();
    let set_c: HashSet<char> = c.chars().collect();

    set_a
        .into_iter()
        .filter(|c| set_b.contains(&c))
        .filter(|c| set_c.contains(&c))
        .next()
        .unwrap()
}
