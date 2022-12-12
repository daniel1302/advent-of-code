use std::vec::Vec;

#[derive(Debug)]
enum ChangeDir {
    Up(String),
    Down,
}

#[derive(Debug)]
enum File {
    File{ size: usize, name: String },
    Dir(String),
}

#[derive(Debug)]
enum Command {
    Unknown,
    Cd(ChangeDir),
    Ls,
}

#[derive(Debug)]
enum TerminalLine {
    Empty,
    Command(Command),
    File(File),
}

fn parse_commands(input: &str) -> Vec<TerminalLine> {
    input.lines().map(|line| {
        if line.get(0..1).unwrap_or("") == "$" {
            let cmd_parts = line.get(2..).unwrap().split(" ").collect::<Vec<&str>>();

            return TerminalLine::Command(match cmd_parts[0] {
                    "ls" => Command::Ls,
                    "cd" => Command::Cd(match cmd_parts[1] {
                        ".." => ChangeDir::Down,
                        child_name => ChangeDir::Up(child_name.to_string()),
                    }),
                    _ => Command::Unknown,
                });
        } 
        else {
            let file_parts = line.split(" ").collect::<Vec<&str>>();
            if file_parts[0] == "dir" {
                return TerminalLine::File(File::Dir(file_parts[1].to_string()))
            } else if file_parts.len() == 2 && file_parts[0].parse::<u32>().is_ok() {
                return TerminalLine::File(File::File { size: file_parts[0].parse::<usize>().unwrap(), name: file_parts[1].to_string() });
            }
        }
        
        TerminalLine::Empty
    })
    .collect::<Vec<TerminalLine>>()
}

fn compute_dirs_size(input: &str) -> Vec<(String, usize)> {
    let terminal_lines = parse_commands(input);
    let mut analyzed_dirs: Vec<(String, usize)> = Vec::new();
    let mut cwd: Vec<(String, usize)> = Vec::new();

    terminal_lines.iter().for_each(|line| {
        match line {
            TerminalLine::Command(cmd) => match cmd {
                Command::Cd(change_dir) => match change_dir {
                    ChangeDir::Up(name) => {
                        cwd.push((name.to_string(), 0));
                    }
                    ChangeDir::Down => {
                        let current_dir = cwd.pop().unwrap();
                        analyzed_dirs.push(current_dir);
                    }
                }
                _ => {}
            }
            TerminalLine::File(f) => match f {
                File::File { size, name } => {
                    cwd.iter_mut().for_each(|dir| dir.1 += size);
                },
                _ => {},
            }
            TerminalLine::Empty => {},
        }
    });

    analyzed_dirs.append(&mut cwd);

    analyzed_dirs
}

pub fn process_part1(input: &str) -> String {
    compute_dirs_size(input).iter().map(|dir| {
        if dir.1 < 100000 { dir.1 } else { 0 }
    }).sum::<usize>().to_string()
}

#[allow(unused)]
pub fn process_part2(input: &str) -> String {
    const FILESYSTEM_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;

    let mut directory_sizes = compute_dirs_size(input)
        .iter()
        .map(|dir| dir.1)
        .collect::<Vec<usize>>();
    
    directory_sizes.sort();

    let root_dir_size = directory_sizes.iter().last().unwrap();

    let required_space: usize = REQUIRED_SPACE - (FILESYSTEM_SPACE - root_dir_size);
    for size in directory_sizes {
        if size > required_space {
            return size.to_string()
        }
    }

    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn part1() {
        assert_eq!(process_part1(INPUT), "95437");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(INPUT), "24933642");
    }
}
