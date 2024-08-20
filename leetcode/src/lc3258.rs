#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "11111".to_owned();
        let k = 1;
        let ans = 15;
        assert_eq!(Solution::count_k_constraint_substrings(s, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
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
        let a = 0;
        let b = n;
        let mut left = 0;
        let mut right = n + 1;
        while left < right {
            let m = (left + right) / 2;
            if ones[m] - ones[a] <= k {
                left = m + 1;
            } else {
                right = m;
            }
        }
        let p = right;
        let mut left = 0;
        let mut right = n + 1;
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
        r += mem[b] - mem[b.min(p)];
        r as i32
    }
}
