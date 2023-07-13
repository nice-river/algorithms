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
    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let mut is_pali = vec![vec![false; n]; n];
        for i in 0..n {
            is_pali[i][i] = true;
        }
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if s[i] == s[j] {
                    if j == i + 1 {
                        is_pali[i][j] = true;
                    } else if is_pali[i + 1][j - 1] {
                        is_pali[i][j] = true;
                    }
                }
            }
        }
        for i in 1..n - 1 {
            for j in i + 1..n {
                if is_pali[0][i - 1] && is_pali[i][j - 1] && is_pali[j][n - 1] {
                    return true;
                }
            }
        }
        false
    }
}
