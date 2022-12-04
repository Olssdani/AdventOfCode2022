use parse_display::{Display, FromStr};
use std::{error::Error, fs};

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("{f1}-{t1},{f2}-{t2}")]
struct Pairs {
    #[from_str(default)]
    f1: i32,
    #[from_str(default)]
    f2: i32,
    #[from_str(default)]
    t1: i32,
    #[from_str(default)]
    t2: i32,
}

impl Pairs {
    fn contain(&self) -> bool {
        let mut res = self.f1 >= self.f2 && self.t1 <= self.t2;
        res |= self.f2 >= self.f1 && self.t2 <= self.t1;
        res
    }

    fn overlap(&self) -> bool {
        let mut res = self.t1 >= self.f2 && self.f1 < self.f2;
        res |= self.t2 >= self.f1 && self.f1 > self.f2;
        res
    }
}

fn run(input: String) -> Result<(i32, i32), Box<dyn Error>> {
    let count_a = input
        .lines()
        .filter(|f| f.parse::<Pairs>().unwrap().contain())
        .count();

    let count_b = input
        .lines()
        .filter(|f| {
            let pair = f.parse::<Pairs>().unwrap();
            pair.contain() || pair.overlap()
        })
        .count();

    Ok((count_a as i32, count_b as i32))
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("day_4/input.txt")?;

    let (a, b) = run(contents)?;
    println!("Res A: {a} Res B: {b}");
    Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let test_data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        .to_string();

    assert_eq!(run(test_data)?, (2, 4));

    Ok(())
}
