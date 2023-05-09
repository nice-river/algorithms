#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let board = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];
        let board = board
            .to_vec()
            .into_iter()
            .map(|row| {
                let row = row.to_vec();
                row.into_iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let ans = true;
        assert_eq!(Solution::is_valid_sudoku(board), ans);
    }

    #[test]
    fn test1() {
        let board = [
            [".", ".", ".", ".", "5", ".", ".", "1", "."],
            [".", "4", ".", "3", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", "3", ".", ".", "1"],
            ["8", ".", ".", ".", ".", ".", ".", "2", "."],
            [".", ".", "2", ".", "7", ".", ".", ".", "."],
            [".", "1", "5", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", "2", ".", ".", "."],
            [".", "2", ".", "9", ".", ".", ".", ".", "."],
            [".", ".", "4", ".", ".", ".", ".", ".", "."],
        ];
        let board = board
            .to_vec()
            .into_iter()
            .map(|row| {
                let row = row.to_vec();
                row.into_iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let ans = false;
        assert_eq!(Solution::is_valid_sudoku(board), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = 9;
        let mut cnt = vec![0; 10];

        for i in 0..9 {
            cnt.iter_mut().for_each(|e| *e = 0);
            for j in 0..9 {
                if board[i][j] != '.' {
                    let x = board[i][j].to_digit(10).unwrap() as usize;
                    if cnt[x] != 0 {
                        return false;
                    }
                    cnt[x] = 1;
                }
            }
        }

        for i in 0..9 {
            cnt.iter_mut().for_each(|e| *e = 0);
            for j in 0..9 {
                if board[j][i] != '.' {
                    let x = board[j][i].to_digit(10).unwrap() as usize;
                    if cnt[x] != 0 {
                        return false;
                    }
                    cnt[x] = 1;
                }
            }
        }

        for i in 0..9 {
            cnt.iter_mut().for_each(|e| *e = 0);
            for j in 0..9 {
                let p = i / 3 * 3 + j / 3;
                let q = i % 3 * 3 + j % 3;
                if board[p][q] != '.' {
                    let x = board[p][q].to_digit(10).unwrap() as usize;
                    if cnt[x] != 0 {
                        return false;
                    }
                    cnt[x] = 1;
                }
            }
        }

        true
    }
}
