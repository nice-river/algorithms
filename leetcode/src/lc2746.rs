#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let words = vec!["aaa".to_owned(), "aba".to_owned(), "c".to_owned()];
        let ans = 6;
        assert_eq!(Solution::minimize_concatenated_length(words), ans);
    }

    #[test]
    fn test1() {
        let words = vec!["aab".to_owned(), "ba".to_owned()];
        let ans = 4;
        assert_eq!(Solution::minimize_concatenated_length(words), ans);
    }

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
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        let words = words
            .into_iter()
            .map(|s| {
                s.into_bytes()
                    .into_iter()
                    .map(|b| (b - b'a') as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        const N: usize = 26;
        let mut dp = vec![vec![vec![i32::MAX; N]; N]; words.len()];
        dp[0][words[0][0]][words[0][words[0].len() - 1]] = words[0].len() as i32;
        for i in 1..words.len() {
            for j in 0..N {
                for k in 0..N {
                    if dp[i - 1][j][k] != i32::MAX {
                        let x = words[i][0];
                        let y = *words[i].last().unwrap();
                        dp[i][j][y] = dp[i][j][y].min(
                            dp[i - 1][j][k] + words[i].len() as i32 - if k == x { 1 } else { 0 },
                        );
                        dp[i][x][k] = dp[i][x][k].min(
                            dp[i - 1][j][k] + words[i].len() as i32 - if j == y { 1 } else { 0 },
                        );
                    }
                }
            }
        }

        let mut ans = i32::MAX;
        for i in 0..N {
            for j in 0..N {
                ans = ans.min(dp[words.len() - 1][i][j]);
            }
        }
        ans
    }
}
