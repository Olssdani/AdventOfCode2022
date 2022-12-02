use parse_display::{Display, FromStr};
use std::{error::Error, fs};

#[derive(FromStr, Display, Debug, PartialEq)]
enum Opponent {
    A,
    B,
    C,
}

#[derive(FromStr, Display, Debug, PartialEq)]
enum MyChoice {
    X,
    Y,
    Z,
}

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("{opponent} {my_choice}")]
struct Selection {
    #[from_str(default)]
    opponent: Opponent,

    #[from_str(default)]
    my_choice: MyChoice,
}

impl Selection {
    fn score_a(self) -> i32 {
        match self.my_choice {
            MyChoice::X => match self.opponent {
                Opponent::A => 1 + 3,
                Opponent::B => 1 + 0,
                Opponent::C => 1 + 6,
            },
            MyChoice::Y => match self.opponent {
                Opponent::A => 2 + 6,
                Opponent::B => 2 + 3,
                Opponent::C => 2 + 0,
            },
            MyChoice::Z => match self.opponent {
                Opponent::A => 3 + 0,
                Opponent::B => 3 + 6,
                Opponent::C => 3 + 3,
            },
        }
    }

    fn score_b(self) -> i32 {
        match self.my_choice {
            MyChoice::X => match self.opponent {
                Opponent::A => 3 + 0,
                Opponent::B => 1 + 0,
                Opponent::C => 2 + 0,
            },
            MyChoice::Y => match self.opponent {
                Opponent::A => 1 + 3,
                Opponent::B => 2 + 3,
                Opponent::C => 3 + 3,
            },
            MyChoice::Z => match self.opponent {
                Opponent::A => 2 + 6,
                Opponent::B => 3 + 6,
                Opponent::C => 1 + 6,
            },
        }
    }
}

fn calc(input: String) -> (i32, i32) {
    let a = input
        .lines()
        .map(|l| l.parse::<Selection>().unwrap().score_a())
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();

    let b = input
        .lines()
        .map(|l| l.parse::<Selection>().unwrap().score_b())
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();
    (a, b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("day_2/input.txt")?;
    let (a, b) = calc(contents);
    println!("Res A: {a} Res B: {b}");
    Ok(())
}

#[test]
fn test() {
    let test_data = "A Y
B X
C Z"
    .to_string();

    assert_eq!(calc(test_data.clone()), (15, 12));
}
