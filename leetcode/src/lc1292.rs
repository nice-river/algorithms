#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mat = to_vec2d([[1]]);
        let threshold = 1;
        let ans = 1;
        assert_eq!(Solution::max_side_length(mat, threshold), ans);
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
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut s = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                s[i][j] = s[i][j - 1] + mat[i - 1][j - 1];
            }
        }
        for i in 1..=n {
            for j in 1..=m {
                s[i][j] += s[i - 1][j];
            }
        }
        let mut l = 0;
        let mut r = m.min(n) + 1;
        while l + 1 < r {
            let m = (l + r) / 2;
            if Self::check(&s, threshold, m) {
                l = m;
            } else {
                r = m;
            }
        }
        l as i32
    }

    fn check(s: &Vec<Vec<i32>>, threshold: i32, length: usize) -> bool {
        let n = s.len();
        let m = s[0].len();
        for i in length..n {
            for j in length..m {
                if s[i][j] - s[i - length][j] - s[i][j - length] + s[i - length][j - length]
                    <= threshold
                {
                    return true;
                }
            }
        }
        false
    }
}
