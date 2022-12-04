pub fn process_part_1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|elf_pair| elf_pair.split_once(',').unwrap())
        .filter(|(first, second)| {
            let (first_start, first_end): (u32, u32) = first
                .split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap();

            let (second_start, second_end): (u32, u32) = second
                .split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap();

            (first_start >= second_start && first_end <= second_end) || (second_start >= first_start && second_end <= first_end)
        })
        .count();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|elf_pair| elf_pair.split_once(',').unwrap())
        .filter(|(first, second)| {
            let (first_start, first_end): (u32, u32) = first
                .split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap();

            let (second_start, second_end): (u32, u32) = second
                .split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap();

            first_start <= second_end && first_end >= second_start
        })
        .count();

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

        assert_eq!(result, "2");
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./test_input.txt").unwrap();

        let result = process_part_2(&input);

        assert_eq!(result, "4");
    }
}
