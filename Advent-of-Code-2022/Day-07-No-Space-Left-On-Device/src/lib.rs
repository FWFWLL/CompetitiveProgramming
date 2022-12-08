#![feature(iter_intersperse)]

use std::collections::BTreeMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, is_a};
use nom::IResult;
use nom::character::complete::{newline, alpha1, self};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

#[derive(Debug)]
enum Command<'a> {
    ChangeDir(ChangeDir<'a>),
    List(Vec<Files<'a>>),
}

#[derive(Debug)]
enum ChangeDir<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File {
        size: u32,
    },
    Dir(&'a str),
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag("/"), tag(".."), alpha1))(input)?;

    let cmd = match dir {
        "/" => Command::ChangeDir(ChangeDir::Root),
        ".." => Command::ChangeDir(ChangeDir::Up),
        name => Command::ChangeDir(ChangeDir::Down(name)),
    };

    Ok((input, cmd))
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _)) = separated_pair(complete::u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;

    Ok((input, Files::File {size}))
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

    Ok((input, Command::List(files)))
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, cmd) = separated_list1(newline, alt((cd, ls)))(input)?;

    Ok((input, cmd))
}

struct File {
    size: u32,
}

pub fn process_part_1(input: &str) -> String {
    let (_, commands) = commands(input).unwrap();

    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<&str> = vec![];

    commands
        .iter()
        .for_each(|command| {
            match command {
                Command::ChangeDir(ChangeDir::Root) => context.push("/"),
                Command::ChangeDir(ChangeDir::Up) => {
                    context.pop();
                    context.pop();
                },
                Command::ChangeDir(ChangeDir::Down(name)) => {
                    context.push(name);
                    context.push("/");
                },
                Command::List(files) => {
                    directories.entry(context.iter().cloned().collect()).or_insert(vec![]);

                    files
                        .iter()
                        .for_each(|file| {
                            match file {
                                Files::File {size, ..} => {
                                    directories.entry(context.iter().cloned().collect()).and_modify(|vec| {
                                        vec.push(File {size: *size})
                                    });
                                },
                                Files::Dir(_) => (),
                            }
                        });
                },
            }
        });

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();

    directories
        .iter()
        .for_each(|(path, files)| {
            let dirs: Vec<&str> = path.split("/").collect();

            let size: u32 = files
                .iter()
                .map(|File {size, ..}| size)
                .sum();

            (0..dirs.len())
                .into_iter()
                .for_each(|i| {
                    sizes
                        .entry((&dirs[0..i]).iter().cloned().collect())
                        .and_modify(|v| *v += size)
                        .or_insert(size);
                });
        });

    let result: u32 = sizes
        .iter()
        .map(|(_, size)| size)
        .filter(|&&size| size <= 100000)
        .sum();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let (_, commands) = commands(input).unwrap();

    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<&str> = vec![];

    commands
        .iter()
        .for_each(|command| {
            match command {
                Command::ChangeDir(ChangeDir::Root) => context.push("/"),
                Command::ChangeDir(ChangeDir::Up) => {
                    context.pop();
                    context.pop();
                },
                Command::ChangeDir(ChangeDir::Down(name)) => {
                    context.push(name);
                    context.push("/");
                },
                Command::List(files) => {
                    directories.entry(context.iter().cloned().intersperse("/").collect()).or_insert(vec![]);

                    files
                        .iter()
                        .for_each(|file| {
                            match file {
                                Files::File {size, ..} => {
                                    directories.entry(context.iter().cloned().intersperse("/").collect()).and_modify(|vec| {
                                        vec.push(File {size: *size})
                                    });
                                },
                                Files::Dir(_) => (),
                            }
                        });
                },
            }
        });

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();

    directories
        .iter()
        .for_each(|(path, files)| {
            let dirs: Vec<&str> = path.split("/").collect();

            let size: u32 = files
                .iter()
                .map(|File {size, ..}| size)
                .sum();

            (0..dirs.len())
                .into_iter()
                .for_each(|i| {
                    sizes
                        .entry((&dirs[0..=i]).iter().cloned().intersperse("/").collect())
                        .and_modify(|v| *v += size)
                        .or_insert(size);
                });
        });

    let total = 70_000_000;
    let needed = 30_000_000;

    let used = sizes
        .get("")
        .unwrap();

    let free = total - used;
    let to_free = needed - free;

    let result: u32 = sizes
        .iter()
        .filter(|(_, &size)| size > to_free)
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
