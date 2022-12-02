use std::collections::HashMap;

pub fn process_part_1(input: &str) -> String {
    let scoring_sheet: HashMap<&str, u32>= HashMap::from([
        ("B X", 1),
        ("C Y", 2),
        ("A Z", 3),
        ("A X", 4),
        ("B Y", 5),
        ("C Z", 6),
        ("C X", 7),
        ("A Y", 8),
        ("B Z", 9),
    ]);

    let result: u32 = input
        .lines()
        .map(|line| scoring_sheet.get(line).unwrap())
        .sum();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let scoring_sheet: HashMap<&str, u32> = HashMap::from([
        ("B X", 1),
        ("C X", 2),
        ("A X", 3),
        ("A Y", 4),
        ("B Y", 5),
        ("C Y", 6),
        ("C Z", 7),
        ("A Z", 8),
        ("B Z", 9),
    ]);

    let result: u32 = input
        .lines()
        .map(|line| scoring_sheet.get(line).unwrap())
        .sum();

    return result.to_string();
}
