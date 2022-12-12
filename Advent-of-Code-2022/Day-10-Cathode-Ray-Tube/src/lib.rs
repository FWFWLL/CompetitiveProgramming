use nom::{
    *,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline}, multi::separated_list1,
};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, value) = complete::i32(input)?;

    Ok((input, Instruction::Addx(value)))
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Instruction::Noop))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(newline, alt((addx, noop)))(input)?;

    Ok((input, instructions))
}

pub fn process_part_1(input: &str) -> String {
    let instructions = instructions(input).unwrap().1;

    let mut current_cycle = 0;
    let mut register_x = 1;

    let mut signal_strengths: Vec<i32> = vec![];

    instructions.iter().for_each(|instruction| {
        current_cycle += 1;

        if current_cycle % 40 == 20 {signal_strengths.push(current_cycle * register_x)}

        match instruction {
            Instruction::Addx(value) => {
                current_cycle += 1;

                if current_cycle % 40 == 20 {signal_strengths.push(current_cycle * register_x)}

                register_x += value;
            },
            Instruction::Noop => {},
        }
    });

    let result: i32 = signal_strengths
        .iter()
        .sum();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let instructions = instructions(input).unwrap().1;

    let mut position = 0;
    let mut register_x = 1;

    let mut result = String::new();

    instructions.iter().for_each(|instruction| {
        if position == 40 {
            result += "\n";
            position = 0;
        }

        result += if position >= register_x - 1 && position <= register_x + 1 {"#"} else {"."};

        position += 1;

        match instruction {
            Instruction::Addx(value) => {
                if position == 40 {
                    result += "\n";
                    position = 0;
                }

                result += if position >= register_x - 1 && position <= register_x + 1 {"#"} else {"."};

                position += 1;

                register_x += value;
            },
            Instruction::Noop => {},
        }
    });

    result += "\n";

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

        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./test_input.txt").unwrap();

        let result = process_part_2(&input);

        let expected = fs::read_to_string("./test_output.txt").unwrap();

        assert_eq!(result, expected);
    }
}
