use std::collections::BTreeSet;

pub fn process_part_1(input: &str) -> String {
    let result = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|(_, subsequence)| {
            let unique_chars: BTreeSet<&char> = subsequence
                .iter()
                .collect();

            subsequence.len() == unique_chars.len()
        })
        .map(|(i, _)| i + 4)
        .unwrap();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let result = input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
        .find(|(_, subsequence)| {
            let unique_chars: BTreeSet<&char> = subsequence
                .iter()
                .collect();

            subsequence.len() == unique_chars.len()
        })
        .map(|(i, _)| i + 14)
        .unwrap();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(process_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(process_part_1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(process_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(process_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(process_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(process_part_2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(process_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(process_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
