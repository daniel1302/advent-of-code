use std::vec::Vec;

trait FileObject {
    fn name(&self) -> String;
    fn size(&self) -> u32;
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl FileObject for File {
    fn size(&self) -> u32 {
        return self.size;
    }
    fn name(&self) -> String {
        return self.name.clone();
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<Box<Dir>>,
    dirs: Vec<Box<Dir>>,
    files: Vec<File>,
}

impl FileObject for Dir {
    fn size(&self) -> u32 {
        return 0;
    }
    fn name(&self) -> String {
        return self.name.clone();
    }
}

impl Dir {
    pub fn new(name: String) -> Self {
        Dir{
            name: name,
            parent: None,
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn add_dir(&self, dir: Box<Dir>) {
        self.dirs.push(dir)
    }
}

#[derive(Debug)]
enum Command {
    Unknown,
    ChangeDir,
    List,
}

#[derive(Debug)]
struct CommandLine {
    cmd: Command,
    args: Vec<String>,
}

#[derive(Debug)]
enum TerminalLine {
    Empty,
    File(File),
    Command(CommandLine),
    Dir(Dir),
}

fn parse_commands(input: &str) -> Vec<TerminalLine> {
    input.lines().map(|line| {
        if line.get(0..1).unwrap_or("") == "$" {
            let mut cmd_parts = line.get(2..).unwrap().split(" ");

            return TerminalLine::Command(CommandLine{
                cmd: match cmd_parts.nth(0).unwrap() {
                    "ls" => Command::List,
                    "cd" => Command::ChangeDir,
                    _ => Command::Unknown,
                },
                args: cmd_parts.map(|arg| arg.to_string()).collect::<Vec<String>>(),
            });
        } 
        else if line.get(0..3).unwrap_or("") == "dir" {
            return TerminalLine::Dir(Dir::new(line.get(3..).unwrap().to_string()));
        } else {
            let file_parts = line.split(" ").collect::<Vec<&str>>();
            if file_parts.len() == 2 && file_parts[0].parse::<u32>().is_ok() {
                return TerminalLine::File(File{
                    name: file_parts[1].to_string(),
                    size: file_parts[0].parse::<u32>().unwrap(),
                });
            }
        }
        
        TerminalLine::Empty
    })
    .collect::<Vec<TerminalLine>>()
}

pub fn process_part1(input: &str) -> String {
    let terminal_lines = parse_commands(input);
    let cwd: Option<Box<Dir>> = None;

    terminal_lines.for_each(|line| {
        match line {
            TerminalLine::Command(command) => {
                match command.cmd {
                    Command::ChangeDir => {
                        if cwd.is_none() {
                            cwd = Some(Box::new(Dir::new(command.args[0])));
                        } else {
                            cwd.
                        }
                    }
                }
            }
        }
    })
}

#[allow(unused)]
pub fn process_part2(input: &str) -> String {
    "result".to_string()
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
        assert_eq!(process_part2(INPUT), "result");
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(process_part2(INPUT), "result");
    }
}
