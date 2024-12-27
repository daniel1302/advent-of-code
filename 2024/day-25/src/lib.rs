use itertools::Itertools;

type Key = (i8, i8, i8, i8, i8);
type Lock = (i8, i8, i8, i8, i8);

enum Item {
    Key(Key),
    Lock(Lock),
}

fn key_fit_lock(lock: &Lock, key: &Key) -> bool {
    lock.0 + key.0 <= 5 && lock.1 + key.1 <= 5 && lock.2 + key.2 <= 5
        && lock.3 + key.3 <= 5 && lock.4 + key.4 <= 5
}

fn parse_part(input: &str) -> Item {
    const EMPTY: u8 = '.' as u8;
    const CODE: u8 = '#' as u8;

    let normalized_input = input.as_bytes();

    let get_height = |col: usize, expected_char: u8| {
        // we store the input characters in the linear vector
        //  - The first line needs to be skipped.
        //  - Each line is 6 characters (5 chars + new line)
        //  - Each column is 6 rows (1 header + 5 data rows)

        // initial_idx = first_line length + col
        let mut idx = 6 + col;

        let mut res = 0;
        // max index = 29
        while idx < 35 {
            if normalized_input[idx] == expected_char {
                res += 1;
            } else {
                break;
            }
            idx += 6;
        }

        res
    };

    if normalized_input[0] == EMPTY {
        Item::Key((
            5 - get_height(0, EMPTY),
            5 - get_height(1, EMPTY),
            5 - get_height(2, EMPTY),
            5 - get_height(3, EMPTY),
            5 - get_height(4, EMPTY),
        ))
    } else { // lock
        Item::Lock((
            get_height(0, CODE),
            get_height(1, CODE),
            get_height(2, CODE),
            get_height(3, CODE),
            get_height(4, CODE),
        ))
    }
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    input
        .trim()
        .split("\n\n")
        .fold((Vec::new(), Vec::new()), |(mut locks_acc, mut keys_acc), lines| {
            match parse_part(lines) {
                Item::Key(k) => {
                    keys_acc.push(k);
                },
                Item::Lock(l) => {
                    locks_acc.push(l);
                }
            }

            (locks_acc, keys_acc)
        })

}


pub fn process_part1(input: &str) -> String {
    let (locks, keys) = parse_input(input);

    locks
        .iter()
        .map(|lock| {
            keys
                .iter()
                .map(|key| {
                    key_fit_lock(lock, key) as i32
                })
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    "Free star here!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}