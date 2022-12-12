
pub fn process_part1(input: &str) -> String {
    let cols = input.lines().last().unwrap().chars().map(|_| 1).sum::<usize>();
    let lines = input.lines().map(|_| 1).sum::<usize>();

    let input_rows = input
        .lines()
        .map(|row| row.chars().map(|num| num.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let input_cols = (0..cols)
        .map(|col| {
            input_rows.iter()
                .map(|line| line[col])
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let visible_trees = (1..cols-1).map(|col| {
        (1..lines-1).map(|row| {
            let current_height = input_rows[row][col];

            let lower_in_row = input_rows[row].iter().map(|&height| height<current_height).collect::<Vec<bool>>();
            let lower_in_col = input_cols[col].iter().map(|&height| height<current_height).collect::<Vec<bool>>();

            if lower_in_col[0..row].iter().all(|&lower| lower) {
                return 1
            } else if lower_in_col[row+1..].iter().all(|&lower| lower) {
                return 1
            } else if lower_in_row[0..col].iter().all(|&lower| lower) {
                return 1
            } else if lower_in_row[col+1..].iter().all(|&lower| lower) {
                return 1
            }
           0
        }).sum::<usize>()
    }).sum::<usize>(); 

    (visible_trees + 2*cols + 2*lines - 4).to_string()
}

pub fn process_part2(input: &str) -> String {
    let cols = input.lines().last().unwrap().chars().map(|_| 1).sum::<usize>();
    let lines = input.lines().map(|_| 1).sum::<usize>();

    let input_rows = input
        .lines()
        .map(|row| row.chars().map(|num| num.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let input_cols = (0..cols)
        .map(|col| {
            input_rows.iter()
                .map(|line| line[col])
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    (1..cols-1).map(|col| {
        (1..lines-1).map(|row| {
            let current_height = input_rows[row][col];

            let mut score = (0usize, 0usize, 0usize, 0usize);

            // top
            for i in (0..row).rev() {
                score.0 += 1;
                if input_cols[col][i] >= current_height {
                    break;
                }
            }
            
            // bottom
            for i in row+1..input_cols[col].len() {
                score.1 += 1;
                if input_cols[col][i] >= current_height {
                    break;
                }
            }

            // left
            for i in (0..col).rev() {
                score.2 += 1;
                if input_rows[row][i] >= current_height {
                    break;
                }
            }

            // right
            for i in col+1..input_cols[row].len() {
                score.3 += 1;
                if input_rows[row][i] >= current_height {
                    break;
                }
            }

            score.0*score.1*score.2*score.3
        }).max().unwrap()
    }).max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1() {
        assert_eq!(process_part1(INPUT), "21");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(INPUT), "8");
    }
}
