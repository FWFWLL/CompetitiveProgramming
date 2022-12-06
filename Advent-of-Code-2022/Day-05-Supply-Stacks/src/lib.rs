use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::streaming::space1;
use nom::sequence::{delimited, preceded};
use nom::character::complete::{self, alpha1, newline, digit1, multispace1};
use nom::multi::{separated_list1, many1};

fn krate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, char) = alt((
        tag("   "),
        delimited(
            complete::char('['),
            alpha1,
            complete::char(']')
        )
    ))(input)?;

    let result = match char {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), krate)(input)?;

    Ok((input, result))
}

#[derive(Debug)]
struct Instruction {
    number: u32,
    from: u32,
    to: u32,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Instruction {
            number,
            from: from - 1,
            to: to - 1
        },
    ))
}

fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Instruction>)> {
    let (input, crates_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, instructions) = separated_list1(newline, instruction)(input)?;
    
    let mut crates_vertical: Vec<Vec<Option<&str>>> = vec![];

    (0..=crates_horizontal.len()).for_each(|_| {
            crates_vertical.push(vec![]);
    });

    crates_horizontal
        .iter()
        .rev()
        .for_each(|crates| {
            crates
                .iter()
                .enumerate()
                .for_each(|(i, krate)| {
                    crates_vertical[i].push(krate.clone());
                });
        });

    let crate_stacks: Vec<Vec<&str>> = crates_vertical
        .iter()
        .map(|stack| {
            stack
                .iter()
                .filter_map(|value| *value)
                .collect()
        })
        .collect();

    Ok((input, (crate_stacks, instructions)))
}

pub fn process_part_1(input: &str) -> String {
    let (_, (mut crate_stacks, instructions)) = crates(input).unwrap();

    instructions
        .iter()
        .for_each(|Instruction {number, from, to}| {
            let len = crate_stacks[*from as usize].len();
            crate_stacks[*from as usize].drain((len - *number as usize)..)
                .rev()
                .collect::<Vec<&str>>()
                .iter()
                .for_each(|krate| {
                    crate_stacks[*to as usize].push(krate);
                })
        });

    let result: String = crate_stacks
        .iter()
        .map(|stack| match stack.last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let (_, (mut crate_stacks, instructions)) = crates(input).unwrap();

    instructions
        .iter()
        .for_each(|Instruction {number, from, to}| {
            let len = crate_stacks[*from as usize].len();
            crate_stacks[*from as usize].drain((len - *number as usize)..)
                .collect::<Vec<&str>>()
                .iter()
                .for_each(|krate| {
                    crate_stacks[*to as usize].push(krate);
                })
        });

    let result: String = crate_stacks
        .iter()
        .map(|stack| match stack.last() {
            Some(c) => c,
            None => "",
        })
        .collect();

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

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./test_input.txt").unwrap();

        let result = process_part_2(&input);

        assert_eq!(result, "MCD");
    }
}
