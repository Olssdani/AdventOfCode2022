use parse_display::{Display, FromStr};
use std::{collections::VecDeque, error::Error, fs};

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("move {amount} from {from} to {to}")]
struct Movement {
    #[from_str(default)]
    amount: i32,
    #[from_str(default)]
    from: i32,
    #[from_str(default)]
    to: i32,
}

fn setup(input: &String) -> (usize, Vec<VecDeque<char>>) {
    let size = ((input.lines().next().unwrap().len()) / 4) + 1;
    let mut piles = Vec::new();
    piles.resize(size, VecDeque::new());

    let mut start_index = 0;
    for (index, line) in input.lines().enumerate() {
        start_index = index + 1;
        if line.is_empty() {
            break;
        }

        for i in 0..size {
            let pos = i * 4;
            let c = &line[pos..(pos + 3)].chars().nth(1).unwrap();
            if !c.is_whitespace() {
                piles[i].push_front(c.clone());
            }
        }
    }

    (start_index, piles)
}

fn run_a(input: &String) -> Result<String, Box<dyn Error>> {
    let (start_index, mut piles) = setup(&input);

    for (index, line) in input.lines().enumerate() {
        if index < start_index {
            continue;
        }
        let command = line.parse::<Movement>()?;

        for _ in 0..command.amount {
            let f = piles[(command.from - 1) as usize].pop_back().unwrap();

            piles[(command.to - 1) as usize].push_back(f);
        }
    }

    let mut res = "".to_string();

    for pile in &mut piles {
        let f = pile.pop_back().unwrap();
        res.push(f.clone());
    }

    Ok(res)
}

fn run_b(input: &String) -> Result<String, Box<dyn Error>> {
    let (start_index, mut piles) = setup(&input);

    for (index, line) in input.lines().enumerate() {
        if index < start_index {
            continue;
        }
        let command = line.parse::<Movement>()?;

        let mut temp = VecDeque::new();
        for _ in 0..command.amount {
            let f = piles[(command.from - 1) as usize].pop_back().unwrap();
            temp.push_front(f);
        }

        for f in temp.iter() {
            piles[(command.to - 1) as usize].push_back(f.clone());
        }
    }

    let mut res = "".to_string();

    for pile in &mut piles {
        let f = pile.pop_back().unwrap();
        res.push(f.clone());
    }
    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("day_5/input.txt")?;

    let a = run_a(&contents)?;
    let b = run_b(&contents)?;

    println!("Res A: {a} Res B: {b}");
    Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let test_data = "    [D]    
[N] [C]    
[Z] [M] [P] 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        .to_string();

    assert_eq!(run_a(&test_data)?, "CMZ".to_string());
    assert_eq!(run_b(&test_data)?, "MCD".to_string());

    Ok(())
}
