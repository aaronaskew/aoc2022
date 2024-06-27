use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::
        complete::{self, newline}
    ,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
enum File {
    Directory { name: String },
    Normal { _name: String, size: u32 },
}

impl File {
    fn size(&self, filesystem: &BTreeMap<String, Vec<File>>, curr_path: &String) -> u32 {
        match self {
            File::Directory { name } => {
                let new_path = format!("{}{}/", curr_path, name);
                filesystem
                    .get(&new_path)
                    .unwrap_or_else(|| panic!("{new_path} should exist in tree"))
                    .iter()
                    .map(|file| file.size(filesystem, &new_path))
                    .sum()
            }
            File::Normal { _name: _, size } => *size,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Cd(Cd),
    Ls(Vec<File>),
}

#[derive(Debug)]
enum Cd {
    Up,
    Down(String),
    Root,
}

fn normal(input: &str) -> IResult<&str, File> {
    let (input, (size, name)) =
        separated_pair(complete::u32, tag(" "), take_till1(|c| c == '\n'))(input)?;
    Ok((
        input,
        File::Normal {
            _name: name.into(),
            size,
        },
    ))
}

fn dir(input: &str) -> IResult<&str, File> {
    let (input, (_, name)) =
        separated_pair(tag("dir"), tag(" "), take_till1(|c| c == '\n'))(input)?;
    Ok((input, File::Directory { name: name.into() }))
}

fn file(input: &str) -> IResult<&str, File> {
    alt((dir, normal))(input)
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls\n")(input)?;
    let (input, files) = separated_list1(newline, file)(input)?;
    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    // dbg!(&input);
    let (input, (_, path)) =
        separated_pair(tag("$ cd"), tag(" "), take_till1(|c| c == '\n'))(input)?;
    // dbg!(&path);
    let operation = match path {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        path => Operation::Cd(Cd::Down(path.into())),
    };

    // dbg!(&operation, &input);
    Ok((input, operation))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    alt((cd, ls))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(newline, operation)(input)
}

fn process(input: &str) -> String {
    let (_, operations) = parse(input).expect("should parse");

    dbg!(&operations);

    let mut current_path = vec![];
    let mut filesystem = BTreeMap::new();

    for operation in operations {
        match operation {
            Operation::Cd(cd_type) => match cd_type {
                Cd::Up => {
                    current_path.pop();
                }
                Cd::Down(path) => {
                    current_path.push(format!("{path}/"));
                }
                Cd::Root => {
                    current_path.push("/".into());
                }
            },
            Operation::Ls(files) => {
                filesystem.insert(current_path.clone().into_iter().collect::<String>(), files);
            }
        }
    }

    dbg!(&filesystem);

    filesystem
        .iter()
        .map(|(path, files)| files.iter().map(|f| f.size(&filesystem, path)).sum::<u32>())
        .filter(|size| *size <= 100000)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "$ cd /
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
7214296 k",
        );
        assert_eq!(result, "95437");
    }
}
