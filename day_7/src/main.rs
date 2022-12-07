use parse_display::{Display, FromStr};
use std::{error::Error, fs};

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("$ cd {directory}")]
struct Cd {
    #[from_str(default)]
    directory: String,
}

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("{size} {file_name}")]
struct File {
    #[from_str(default)]
    size: i32,

    #[from_str(default)]
    file_name: String,
}

fn calc_size(index: usize, input: &Vec<&str>, res: &mut Vec<i32>) -> (usize, i32) {
    let mut sum = 0;
    let mut i = index;
    while i < input.len() {
        let line = input[i];
        if let Ok(cd) = line.parse::<Cd>() {
            if cd.directory == ".." {
                return (i, sum);
            } else {
                let (index, size) = calc_size(i + 1, input, res);
                res.push(size);
                i = index;
                sum += size;
            }
        } else if let Ok(file) = line.parse::<File>() {
            sum += file.size;
        }

        i += 1;
    }
    (input.len(), sum)
}

fn run(input: &String) -> Result<(i32, i32), Box<dyn Error>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut i = 1;
    let mut total_size = 0;
    let mut res = Vec::new();

    while i < lines.len() {
        let line = lines[i];
        if line.parse::<Cd>().is_ok() {
            let (index, size) = calc_size(i + 1, &lines, &mut res);
            i = index;
            res.push(size);
            total_size += size;
        } else if let Ok(file) = line.parse::<File>() {
            total_size += file.size;
        }
        i += 1;
    }

    let sum = res.iter().filter(|f| **f < 100_000).sum::<i32>();

    let needed_space = ((70_000_000 - total_size) - 30_000_000).abs();

    let mut closest_value = i32::MAX;
    for value in res {
        if value > needed_space {
            if value < closest_value {
                closest_value = value;
            }
        }
    }

    Ok((sum, closest_value))
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("day_7/input.txt")?;

    let (a, b) = run(&contents)?;

    println!("Res A: {a} Res B: {b}");
    Ok(())
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let test_data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        .to_string();

    assert_eq!(run(&test_data)?, (95437, 24933642));

    Ok(())
}
