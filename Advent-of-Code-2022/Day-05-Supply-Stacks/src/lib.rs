use std::collections::LinkedList;

pub fn process_part_1(input: &str) -> String {
    let (supply_stacks, instructions) = input
        .split_once("\n\n")
        .unwrap();

    let num_stacks: usize = supply_stacks
        .lines()
        .rev()
        .take(1)
        .next()
        .unwrap()
        .trim()
        .split("   ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<LinkedList<char>> = Vec::<LinkedList<char>>::new();

    for _ in 0..num_stacks {
        stacks.push(LinkedList::new());
    }

    let _: () = supply_stacks
        .replace("    [", "[Ø] [")
        .replace("]    ", "] [Ø]")
        .lines()
        .rev()
        .skip(1)
        .into_iter()
        .map(|line| {
            let binding = line
                .replace(&['[', ']'], "");

            let crates = binding
                .split(' ')
                .enumerate();

            crates.for_each(|(i, cargo)| {
                if cargo == "Ø" {}
                else {
                    stacks[i].push_front(cargo.chars().next().unwrap())
                }
            });
        })
    .collect();

    instructions
        .lines()
        .for_each(|instruction| {
            let a = instruction
                .split(" ");
            
            let amount: usize = a
                .clone()
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            let src: usize = a
                .clone()
                .skip(3)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap() - 1;

            let dst: usize = a
                .clone()
                .skip(5)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap() - 1;

            for _ in 0..amount {
                let src_char = stacks[src].pop_front().unwrap();

                stacks[dst].push_front(src_char);
            }
        });

    let mut result = String::new();
    for mut list in stacks {
        result += list.pop_front().unwrap().to_string().as_str();
    }

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let (supply_stacks, instructions) = input
        .split_once("\n\n")
        .unwrap();

    let num_stacks: usize = supply_stacks
        .lines()
        .rev()
        .take(1)
        .next()
        .unwrap()
        .trim()
        .split("   ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<LinkedList<char>> = Vec::new();

    for _ in 0..num_stacks {
        stacks.push(LinkedList::new());
    }

    let _: () = supply_stacks
        .replace("    [", "[Ø] [")
        .replace("]    ", "] [Ø]")
        .lines()
        .rev()
        .skip(1)
        .into_iter()
        .map(|line| {
            let binding = line
                .replace(&['[', ']'], "");

            let crates = binding
                .split(' ')
                .enumerate();

            crates.for_each(|(i, cargo)| {
                if cargo == "Ø" {}
                else {
                    stacks[i].push_front(cargo.chars().next().unwrap())
                }
            });
        })
    .collect();

    instructions
        .lines()
        .for_each(|instruction| {
            let a = instruction
                .split(" ");
            
            let amount: usize = a
                .clone()
                .skip(1)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            let src: usize = a
                .clone()
                .skip(3)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap() - 1;

            let dst: usize = a
                .clone()
                .skip(5)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap() - 1;

            let mut crates : LinkedList<char> = LinkedList::new();
            for _ in 0..amount {
                crates.push_front(stacks[src].pop_front().unwrap());
            }

            for cargo in crates {
                stacks[dst].push_front(cargo);
            }
        });

    let mut result = String::new();
    for mut list in stacks {
        result += list.pop_front().unwrap().to_string().as_str();
    }

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
