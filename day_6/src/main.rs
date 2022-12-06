use std::{error::Error, fs};

fn run(input: &String) -> Result<(usize, i32), Box<dyn Error>> {
    let chars = input.chars().collect::<Vec<char>>();

    let mut res_index = 0;
    let mut sucess_counter = 4;
    for (index, c) in chars.iter().enumerate() {
        println!("{index} {res_index} {sucess_counter}");
        let slice = &chars[(index + 1)..];
        println!("{slice:?}");
        if let Some(d) = slice.iter().position(|&r| r == *c) {
            println!("d {d}");
            if d < sucess_counter {
                res_index = index;

                continue;
            } else {
                sucess_counter -= 1;
            }
        }

        if sucess_counter == 0 {
            break;
        }
    }

    Ok((res_index, 0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("day_5/input.txt")?;

    let (a, b) = run(&contents)?;

    println!("Res A: {a} Res B: {b}");
    Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let test_data = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    assert_eq!(run(&test_data)?, (5, 0));

    let test_data = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    assert_eq!(run(&test_data)?, (6, 0));

    let test_data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    assert_eq!(run(&test_data)?, (10, 0));

    let test_data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
    assert_eq!(run(&test_data)?, (11, 0));

    Ok(())
}
