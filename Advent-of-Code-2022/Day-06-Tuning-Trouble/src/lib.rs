pub fn process_part_1(input: &str) -> String {
    let mut datastream = input
        .chars();

    let mut position = 4;

    while datastream.next().is_some() {
        let mut subsequence: Vec<char> = datastream
            .clone()
            .take(4)
            .collect();

        subsequence.sort();
        subsequence.dedup();

        position += 1;

        if subsequence.len() == 4 {
            break;
        }
    }

    return position.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let mut datastream = input
        .chars();

    let mut position = 14;

    while datastream.next().is_some() {
        let mut subsequence: Vec<char> = datastream
            .clone()
            .take(14)
            .collect();

        subsequence.sort();
        subsequence.dedup();

        position += 1;

        if subsequence.len() == 14 {
            break;
        }
    }

    return position.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn test_part_1() {
        let input_1 = fs::read_to_string("./test_input_1.txt").unwrap();
        let input_2 = fs::read_to_string("./test_input_2.txt").unwrap();
        let input_3 = fs::read_to_string("./test_input_3.txt").unwrap();
        let input_4 = fs::read_to_string("./test_input_4.txt").unwrap();
        let input_5 = fs::read_to_string("./test_input_5.txt").unwrap();

        let result_1 = process_part_1(&input_1);
        let result_2 = process_part_1(&input_2);
        let result_3 = process_part_1(&input_3);
        let result_4 = process_part_1(&input_4);
        let result_5 = process_part_1(&input_5);

        assert_eq!(result_1, "7");
        assert_eq!(result_2, "5");
        assert_eq!(result_3, "6");
        assert_eq!(result_4, "10");
        assert_eq!(result_5, "11");
    }

    #[test]
    fn test_part_2() {
        let input_1 = fs::read_to_string("./test_input_1.txt").unwrap();
        let input_2 = fs::read_to_string("./test_input_2.txt").unwrap();
        let input_3 = fs::read_to_string("./test_input_3.txt").unwrap();
        let input_4 = fs::read_to_string("./test_input_4.txt").unwrap();
        let input_5 = fs::read_to_string("./test_input_5.txt").unwrap();

        let result_1 = process_part_2(&input_1);
        let result_2 = process_part_2(&input_2);
        let result_3 = process_part_2(&input_3);
        let result_4 = process_part_2(&input_4);
        let result_5 = process_part_2(&input_5);

        assert_eq!(result_1, "19");
        assert_eq!(result_2, "23");
        assert_eq!(result_3, "23");
        assert_eq!(result_4, "29");
        assert_eq!(result_5, "26");
    }
}
