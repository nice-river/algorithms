#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let n = 8;
        let mut a = 0;
        let mut b = 0;
        let mut ans = 0;
        for i in 0..n {
            for j in 0..n {
                if board[i][j] == 'R' {
                    a = i;
                    b = j;
                }
            }
        }
        for k in b + 1..n {
            if board[a][k] == 'p' {
                ans += 1;
                break;
            } else if board[a][k] == 'B' {
                break;
            }
        }
        for k in 0..b {
            if board[a][k] == 'p' {
                ans += 1;
                break;
            } else if board[a][k] == 'B' {
                break;
            }
        }
        for k in a + 1..n {
            if board[k][b] == 'p' {
                ans += 1;
                break;
            } else if board[k][b] == 'B' {
                break;
            }
        }
        for k in 0..a {
            if board[k][b] == 'p' {
                ans += 1;
                break;
            } else if board[k][b] == 'B' {
                break;
            }
        }
        ans
    }
}
