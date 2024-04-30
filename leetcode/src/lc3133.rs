#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 2;
        let x = 7;
        let ans = 15;
        assert_eq!(Solution::min_end(n, x), ans);

        let n = 3;
        let x = 3;
        let ans = 11;
        assert_eq!(Solution::min_end(n, x), ans);

        let n = 2;
        let x = 4;
        let ans = 5;
        assert_eq!(Solution::min_end(n, x), ans);
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_end(mut n: i32, mut x: i32) -> i64 {
        let mut x_bit = vec![];
        let mut n_bit = vec![];
        while x != 0 {
            x_bit.push(x % 2);
            x /= 2;
        }
        n -= 1;
        while n != 0 {
            n_bit.push(n % 2);
            n /= 2;
        }
        let mut p = 0;
        let mut i = 0;
        while i < x_bit.len() && p < n_bit.len() {
            while i < x_bit.len() && x_bit[i] == 1 {
                i += 1;
            }
            if i >= x_bit.len() {
                break;
            }
            let mut j = i + 1;
            while j < x_bit.len() && x_bit[j] == 0 {
                j += 1;
            }
            let t = (j - i).min(n_bit.len() - p);
            x_bit[i..i + t].copy_from_slice(&n_bit[p..p + t]);
            p += j - i;
            i = j + 1;
        }
        if p < n_bit.len() {
            x_bit.extend_from_slice(&n_bit[p..]);
        }
        let mut ans = 0i64;
        for b in x_bit.into_iter().rev() {
            ans = ans * 2 + b as i64;
        }
        ans
    }
}
