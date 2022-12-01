pub fn process_part_1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let mut result: Vec<u32> = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    result.sort_by(|a, b| b.cmp(a));

    let tri_max_sum: u32 = result
        .iter()
        .take(3)
        .sum::<u32>();

    return tri_max_sum.to_string();
}
