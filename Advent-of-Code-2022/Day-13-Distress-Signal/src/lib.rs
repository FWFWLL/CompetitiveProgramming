use std::cmp::Ordering;

use nom::*;
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::{self, newline};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair};

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Integer(b)) => a.cmp(&vec![Packet::Integer(*b)]),
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::Integer(a), Self::List(b)) => vec![Packet::Integer(*a)].cmp(&b),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a == b,
            (Self::List(a), Self::Integer(b)) => a == &vec![Packet::Integer(*b)],
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Integer(a), Self::List(b)) => &vec![Packet::Integer(*a)] == b,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

fn packet(input: &str) -> IResult<&str, Packet> {
    let (input, packet) = alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(|list| Packet::List(list)),
        complete::u32.map(|integer| Packet::Integer(integer)),
    ))(input)?;

    Ok((input, packet))
}

fn packet_pair(input: &str) -> IResult<&str, Pair> {
    let (input, pair) = separated_pair(packet, newline, packet)(input)?;
    let (input, _) = newline(input)?;

    let packet_pair = Pair {
        left: pair.0,
        right: pair.1,
    };

    Ok((input, packet_pair))
}

fn parse_signal(input: &str) -> IResult<&str, Vec<Pair>> {
    let (input, packet_pairs) = separated_list1(newline, packet_pair)(input)?;

    Ok((input, packet_pairs))
}

pub fn process_part_1(input: &str) -> String {
    let distress_signal = parse_signal(input).unwrap().1;

    let result: u32 = distress_signal
        .iter()
        .enumerate()
        .filter_map(|(i, Pair {left, right})| {
            match left.cmp(right) {
                Ordering::Less => Some(i as u32),
                Ordering::Equal => panic!("How are they equal!?"),
                Ordering::Greater => None,
            }
        })
        .map(|i| i + 1)
        .sum();

    return result.to_string();
}

pub fn process_part_2(input: &str) -> String {
    let distress_signal = parse_signal(input).unwrap().1;

    let divider_packet_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_packet_6 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    let mut packets: Vec<&Packet> = distress_signal
        .iter()
        .flat_map(|Pair {left, right}| [left, right])
        .chain([&divider_packet_2, &divider_packet_6])
        .collect();

    packets.sort();

    let index_2 = packets
        .iter()
        .enumerate()
        .find_map(|(i, &packet)| if packet == &divider_packet_2 {Some(i + 1)} else {None})
        .unwrap();

    let index_6 = packets
        .iter()
        .enumerate()
        .find_map(|(i, &packet)| if packet == &divider_packet_6 {Some(i + 1)} else {None})
        .unwrap();

    let result = index_2 * index_6;

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT);

        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_2() {
        let result = process_part_2(INPUT);

        assert_eq!(result, "140");
    }
}

