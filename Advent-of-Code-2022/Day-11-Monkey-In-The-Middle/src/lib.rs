use std::collections::VecDeque;

use nom::*;
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::{newline, self, multispace1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};


#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operation {
    Add((Value, Value)),
    Mul((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    on_true: u64,
    on_false: u64,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

impl Monkey {
    fn inspect(&mut self, relief: bool, magic_trick: u64) -> u64 {
        self.inspections += 1;

        let item = self.items.pop_front().unwrap();

        let worry_level = match &self.operation {
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let result = num_a + num_b;

                result % magic_trick
            },
            Operation::Mul((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                let result = num_a * num_b;

                result % magic_trick
            }
        };

        if relief {worry_level / 3} else {worry_level}
    }

    fn test(&self, item: u64) -> u64 {
        if item % self.test.divisible == 0 {self.test.on_true} else {self.test.on_false}
    }
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, on_true) = preceded(tag("If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, on_false) = preceded(tag("If false: throw to monkey "), complete::u64)(input)?;

    Ok((input, Test {divisible, on_true, on_false}))
}

fn value(input: &str) -> IResult<&str, Value> {
    let (input, value) = alt((tag("old").map(|_| Value::Old), complete::u64.map(|num| Value::Num(num))))(input)?;

    Ok((input, value))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value_1) = value(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("+"), tag("*"))), multispace1)(input)?;
    let (input, value_2) = value(input)?;

    let operation = match operator {
        "+" => Operation::Add((value_1, value_2)),
        "*" => Operation::Mul((value_1, value_2)),
        _ => todo!(),
    };

    Ok((input, operation))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(tag("Starting items: "), separated_list1(tag(", "), complete::u64))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;
    let (input, _) = newline(input)?;

    Ok((input, Monkey {items: VecDeque::from(items), operation, test, inspections: 0}))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(newline, monkey)(input)?;

    Ok((input, monkeys))
}

pub fn process_part_1(input: &str) -> String {
    let mut monkeys = parse_monkeys(input).unwrap().1;

    let magic_trick: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product();

    (0..20).for_each(|_| {
        (0..monkeys.len()).for_each(|monkey_index| {
            (0..monkeys[monkey_index].items.len()).for_each(|_| {
                let monkey = monkeys
                    .get_mut(monkey_index)
                    .unwrap();

                let item = monkey.inspect(true, magic_trick);
                let recipient = monkey.test(item);

                monkeys
                    .get_mut(recipient as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            });
        })
    });

    monkeys.sort_by_key(|monkey| monkey.inspections);

    let result: u64 = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspections)
        .product();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let mut monkeys = parse_monkeys(input).unwrap().1;

    let magic_trick: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product();

    (0..10_000).for_each(|_| {
        (0..monkeys.len()).for_each(|monkey_index| {
            (0..monkeys[monkey_index].items.len()).for_each(|_| {
                let monkey = monkeys
                    .get_mut(monkey_index)
                    .unwrap();

                let item = monkey.inspect(false, magic_trick);
                let recipient = monkey.test(item);

                monkeys
                    .get_mut(recipient as usize)
                    .unwrap()
                    .items
                    .push_back(item);
            });
        })
    });

    monkeys.sort_by_key(|monkey| monkey.inspections);

    let result: u64 = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspections)
        .product();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT);

        let expected = "10605";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let result = process_part_2(INPUT);

        let expected = "2713310158";

        assert_eq!(result, expected);
    }
}

