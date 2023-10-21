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

use crate::*;

struct Solution {}

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let mut ans = 0;
        let mut odds = vec![];
        let mut evens = vec![];
        let mut k = num;
        while k != 0 {
            let d = k % 10;
            if d % 2 == 0 {
                evens.push(d);
            } else {
                odds.push(d);
            }
            k /= 10;
        }
        odds.sort();
        evens.sort();
        let mut p = 0;
        let mut q = 0;
        let mut base = 1;
        k = num;
        while k != 0 {
            let d = k % 10;
            if d % 2 == 0 {
                ans += evens[p] * base;
                p += 1;
            } else {
                ans += odds[q] * base;
                q += 1;
            }
            base *= 10;
            k /= 10;
        }
        ans
    }
}
