use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone)]
enum FileSystemNode {
    // index, size
    File(u32),
    Free,
}

fn parse_input(input: &str) -> Vec<FileSystemNode> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10).unwrap()
        })
        .collect::<Vec<u32>>()
        .chunks(2)
        .enumerate()
        .map(|(idx, block_info)| {
            match block_info.len() {
                0 => vec![],
                1 => vec![
                        vec![FileSystemNode::File(idx as u32); block_info[0] as usize]
                    ],
                _ => vec![
                        vec![FileSystemNode::File(idx as u32); block_info[0] as usize],
                        vec![FileSystemNode::Free; block_info[1] as usize]
                    ]
            }
        })
        .flatten()
        .flatten()
        .collect()
}

pub fn process_part1(input: &str) -> String {
    let disk = parse_input(input);
    let mut files_on_disk: VecDeque<&FileSystemNode> = disk
        .iter()
        .filter(|block| !matches!(block, FileSystemNode::Free))
        .collect();

    let all_files_count = files_on_disk.len();

    let organized_disk: Vec<&FileSystemNode> = disk
        .iter()
        .fold( vec![], |acc, fs_node| {
            let mut acc: Vec<&FileSystemNode> = acc.clone();           

            match fs_node {
                FileSystemNode::Free => {
                    acc.push(files_on_disk.pop_back().unwrap_or(&FileSystemNode::Free));
                },
                FileSystemNode::File(_) => { acc.push(fs_node); }
            }

            acc
        })
        .iter()
        .take(disk.len() - (all_files_count - files_on_disk.len()))
        .map(|n| *n)
        .collect();

    organized_disk
        .iter()
        .enumerate()
        .map(|(pos, &fs_node)| {    
            match *fs_node {
                FileSystemNode::Free => 0,
                FileSystemNode::File(v) => {
                    (pos * v  as usize) as u128
                }
            }
        })
        .sum::<u128>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq)]
enum FsBlockKind {
    Free,
    File,
}

#[derive(Debug, Clone)]
struct FsBlock {
    val: usize,
    size: usize,
    kind: FsBlockKind,
}

impl FsBlock {
    fn checksum_at(&self, idx: u128) -> u128 {
        if self.size < 1 {
            return 0
        }

        (0..self.size)
            .fold(0 as u128, |acc, cur_idx| {
                acc + self.val as u128 * (idx + cur_idx as u128)
            })
    }
}

impl Into<String> for FsBlock {
    fn into(self) -> String {
        match self.kind {
            FsBlockKind::File => {
                vec![self.val.to_string(); self.size].iter().map(|x| x.as_str()).collect()
            },
            FsBlockKind::Free => {
                vec![".".to_owned(); self.size].iter().map(|x| x.as_str()).collect()
            }
        }
    }
}

fn parse_input2(input: &str) -> Vec<FsBlock> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10).unwrap()
        })
        .collect::<Vec<u32>>()
        .chunks(2)
        .enumerate()
        .map(|(idx, block_info)| {
            match block_info.len() {
                0 => vec![],
                1 => vec![
                        FsBlock { val: idx, size: block_info.get(0).unwrap().to_owned() as usize, kind: FsBlockKind::File }
                    ],
                _ => vec![
                        FsBlock { val: idx, size: block_info.get(0).unwrap().to_owned() as usize, kind: FsBlockKind::File },
                        FsBlock { val: idx, size: block_info.get(1).unwrap().to_owned() as usize, kind: FsBlockKind::Free },
                    ]
            }
        })
        .flatten()
        .collect()
}

pub fn process_part2(input: &str) -> String {
    let disk = parse_input2(input);

    let mut result = disk.clone();

    let find_space_for_file = |fs_snapshot: &Vec<FsBlock>, req_file_size: usize, val: usize| -> Option<usize> {
        for (idx, fs_block) in fs_snapshot.iter().enumerate() {
            if let FsBlockKind::Free = fs_block.kind {
                if fs_block.size >= req_file_size && val > fs_block.val {
                    return Some(idx)
                }
            }
        }
        return None
    };
    
    let move_file = |fs_snapshot: &mut Vec<FsBlock>, from: usize, to: usize| {
        let fs_block = fs_snapshot.remove(from);

        fs_snapshot.insert(from, FsBlock { val: fs_block.val, size: fs_block.size, kind: FsBlockKind::Free });
        if fs_block.size == fs_snapshot.get(to).unwrap().size {
            fs_snapshot.remove(to);
        } else {
            fs_snapshot.get_mut(to).unwrap().size -= fs_block.size;
        }        
        fs_snapshot.insert(to, fs_block);
    };

    let find_pos_for = |fs_snapshot: &Vec<FsBlock>, block: &FsBlock| {
        fs_snapshot
            .iter()
            .enumerate()
            .find(|(_, cur_block )| {
                cur_block.size == block.size && cur_block.kind == block.kind && cur_block.val == block.val
            }).unwrap().0
    };

    disk
        .iter()
        .rev()
        .for_each(|fs_block| {
            match fs_block.kind {
                FsBlockKind::Free => { return },
                FsBlockKind::File => {
                    let free_slot = find_space_for_file(&result, fs_block.size, fs_block.val);
                    let real_pos_from = find_pos_for(&result, &fs_block);

                    if free_slot.is_some() {
                        move_file(&mut result, real_pos_from, free_slot.unwrap());
                    }
                }
            }
        });

    let mut current_fs_size = 0;
    let mut result_checksum = 0;
    for block in result {
        if let FsBlockKind::File = block.kind {
            result_checksum += block.checksum_at(current_fs_size);
        }
        current_fs_size += block.size as u128;
    }

    result_checksum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}