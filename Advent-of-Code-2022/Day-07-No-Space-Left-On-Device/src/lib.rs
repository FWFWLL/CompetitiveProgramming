use std::collections::BTreeMap;

use nom::{
    IResult,
    bytes::complete::{
        tag,
        is_a
    },
    branch::alt,
    character::complete::{
        alpha1,
        self,
        newline
    },
    sequence::separated_pair,
    multi::separated_list1
};

enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

enum Files<'a> {
    File(u32),
    Dir(&'a str),
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag("/"), tag(".."), alpha1))(input)?;

    let cmd = match dir {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        name => Command::Cd(Cd::Down(name)),
    };

    Ok((input, cmd))
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _)) = separated_pair(complete::u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;

    Ok((input, Files::File(size)))
}

fn dir(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, dir)))(input)?;

    Ok((input, Command::Ls(files)))
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, cmd) = separated_list1(newline, alt((cd, ls)))(input)?;

    Ok((input, cmd))
}

fn calculate_sizes<'a>((mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>), command: &'a Command) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        Command::Cd(Cd::Root) => {
            context.push("");
        },
        Command::Cd(Cd::Up) => {
            context.pop();
        },
        Command::Cd(Cd::Down(name)) => {
            context.push(name);
        },
        Command::Ls(files) => {
            let sum: u32 = files
                .iter()
                .filter_map(|file| if let Files::File(size) = file {Some(size)} else {None})
                .sum();

            (0..context.len()).for_each(|i| {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            });
        }
    };

    (context, sizes)
}

pub fn process_part_1(input: &str) -> String {
    let commands = commands(input).unwrap().1;

    let (_, sizes) = commands
        .iter()
        .fold((vec![], BTreeMap::new()), calculate_sizes);

    let result: u32 = sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let commands = commands(input).unwrap().1;

    let (_, sizes) = commands
        .iter()
        .fold((vec![], BTreeMap::new()), calculate_sizes);

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let used_space = sizes.get(&vec![""]).unwrap();
    let free_space = total_space - used_space;
    let free_space_needed = needed_space - free_space;

    let result: u32 = sizes
        .iter()
        .filter(|(_, &size)| size > free_space_needed)
        .map(|(_, size)| *size)
        .min()
        .unwrap();

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

        assert_eq!(result, "95437");
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("./test_input.txt").unwrap();

        let result = process_part_2(&input);

        assert_eq!(result, "24933642");
    }
}
