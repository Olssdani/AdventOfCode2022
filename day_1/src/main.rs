use std::{collections::HashMap, error::Error, fs};

fn main() {
    let contents = fs::read_to_string("day_1/input.txt").expect("Error reading file");

    let lines = contents
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut santa_helpers: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut index = 0;

    for line in lines {
        if line.is_empty() {
            index += 1;
        } else {
            santa_helpers
                .entry(index)
                .or_insert(Vec::new())
                .push(line.parse::<i32>().expect("Error in input data"));
        }
    }

    let max_food = santa_helpers
        .iter()
        .map(|(_, v)| v.iter().sum::<i32>())
        .max()
        .unwrap();

    let mut food: Vec<i32> = santa_helpers
        .iter()
        .map(|(_, v)| v.iter().sum::<i32>())
        .collect();
    food.sort_by(|a, b| b.partial_cmp(a).unwrap());

    println!("Max {max_food}");
    println!("Top three max {:?}", food[0] + food[1] + food[2]);
}
