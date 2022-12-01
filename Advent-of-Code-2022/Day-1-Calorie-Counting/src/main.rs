use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let mut calories: Vec<i32> = Vec::new();

    let file = File::open("input")
        .unwrap();

    let buffered_reader = BufReader::new(file);

    let mut sum = 0;
    for line in buffered_reader.lines() {
        if let Ok(calorie) = line.unwrap().parse::<i32>() {
            sum += calorie;
        } else {
            calories.push(sum);

            sum = 0;
        }
    }

    let max = calories
        .iter()
        .max()
        .unwrap();

    println!("{max}");

    calories.sort_by(|a, b| b.cmp(a));
    let tri_max = calories[0] + calories[1] + calories[2];

    println!("{tri_max}");
}
