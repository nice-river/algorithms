#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let n = board.len();
        let m = board[0].len();
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'X' {
                    if (i == 0 || board[i - 1][j] == '.') && (j == 0 || board[i][j - 1] == '.') {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
