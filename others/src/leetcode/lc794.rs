#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let board = vec!["XOX", "OOX", "XO "];
        let board = board.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let ans = true;
        assert_eq!(Solution::valid_tic_tac_toe(board), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let board = board
            .into_iter()
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();
        let mut x = 0;
        let mut o = 0;
        for row in board.iter() {
            for &cell in row.iter() {
                if cell == b'X' {
                    x += 1;
                } else if cell == b'O' {
                    o += 1;
                }
            }
        }
        if x < o || x > o + 1 {
            return false;
        }
        let (p, q) = Self::check(&board);
        match (p, q) {
            (0, 0) => true,
            (0, 1) => x == o,
            (1, 0) | (2, 0) => x == o + 1,
            _ => false,
        }
    }

    fn check(board: &Vec<Vec<u8>>) -> (i32, i32) {
        let mut x = 0;
        let mut o = 0;
        for i in 0..3 {
            let mut f = true;
            for j in 1..3 {
                if board[i][j] != board[i][j - 1] {
                    f = false;
                    break;
                }
            }
            if f {
                if board[i][0] == b'X' {
                    x += 1;
                } else if board[i][0] == b'O' {
                    o += 1;
                }
            }
            f = true;
            for j in 1..3 {
                if board[j][i] != board[j - 1][i] {
                    f = false;
                    break;
                }
            }
            if f {
                if board[0][i] == b'X' {
                    x += 1;
                } else if board[0][i] == b'O' {
                    o += 1;
                }
            }
        }
        if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
            if board[1][1] == b'X' {
                x += 1;
            } else if board[1][1] == b'O' {
                o += 1;
            }
        }

        if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
            if board[1][1] == b'X' {
                x += 1;
            } else if board[1][1] == b'O' {
                o += 1;
            }
        }

        (x, o)
    }
}
