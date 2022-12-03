use std::{error::Error, fs};

fn is_in_string(data: &[&str]) -> Option<char> {
    for c in data[0].to_string().chars() {
        let mut found = true;
        for i in 1..=2 {
            found &= data[i].find(c).is_some();
        }

        if found {
            return Some(c);
        }
    }

    None
}

fn char_to_res(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

fn run(input: String) -> (i32, i32) {
    let mut priority_a = 0;
    let mut priority_b = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    for line in &lines {
        let (a, b) = line.split_at(line.len() / 2);

        'inner: for c in a.to_string().chars() {
            if let Some(_) = b.find(c) {
                priority_a += char_to_res(c);

                break 'inner;
            }
        }
    }

    for i in (0..lines.len()).step_by(3) {
        let line = &lines[i..(i + 3)];
        if let Some(c) = is_in_string(line) {
            priority_b += char_to_res(c);
        }
    }

    (priority_a as i32, priority_b as i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("day_3/input.txt")?;
    let (a, b) = run(contents);

    println!("Res A: {a} Res B: {b}");
    Ok(())
}

#[test]
fn test() {
    let test_data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        .to_string();

    assert_eq!(run(test_data.clone()), (157, 70));
}
