#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "0001111".to_owned();
        let k = 2;
        let queries = to_vec!([[0, 6]]);
        let ans = vec![26];
        assert_eq!(Solution::count_k_constraint_substrings(s, k, queries), ans);
    }

    #[test]
    fn test1() {
        let s = "010101".to_owned();
        let k = 1;
        let queries = to_vec!([[0, 5], [1, 4], [2, 3]]);
        let ans = vec![15, 9, 3];
        assert_eq!(Solution::count_k_constraint_substrings(s, k, queries), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let s = s.as_bytes();
        let n = s.len();
        let mut ones = vec![0; n + 1];
        let mut zeros = vec![0; n + 1];
        for i in 0..n {
            ones[i + 1] = ones[i] + if s[i] == b'1' { 1 } else { 0 };
            zeros[i + 1] = zeros[i] + if s[i] == b'0' { 1 } else { 0 };
        }
        let mut mem = vec![0; n + 1];
        for i in (0..n).rev() {
            let mut left = 0;
            let mut right = i + 1;
            while left < right {
                let m = (left + right) / 2;
                if ones[i + 1] - ones[m] <= k {
                    right = m;
                } else {
                    left = m + 1;
                }
            }
            let p = right;
            let mut left = 0;
            let mut right = i + 1;
            while left < right {
                let m = (left + right) / 2;
                if zeros[i + 1] - zeros[m] <= k {
                    right = m;
                } else {
                    left = m + 1;
                }
            }
            let p = p.min(right);
            mem[i + 1] = i - p + 1;
        }
        for i in 0..n {
            mem[i + 1] += mem[i];
        }
        let mut ans = vec![];
        for query in queries {
            let a = query[0] as usize;
            let b = query[1] as usize;
            let mut left = query[0] as usize;
            let mut right = query[1] as usize + 2;
            while left < right {
                let m = (left + right) / 2;
                if ones[m] - ones[a] <= k {
                    left = m + 1;
                } else {
                    right = m;
                }
            }
            let p = right;
            let mut left = query[0] as usize;
            let mut right = query[1] as usize + 2;
            while left < right {
                let m = (left + right) / 2;
                if zeros[m] - zeros[a] <= k {
                    left = m + 1;
                } else {
                    right = m;
                }
            }
            let p = p.max(right) - 1;
            let sz = p - a;
            let mut r = (sz + 1) * sz / 2;
            r += mem[b + 1] - mem[(b + 1).min(p)];
            ans.push(r as i64);
        }
        ans
    }
}
