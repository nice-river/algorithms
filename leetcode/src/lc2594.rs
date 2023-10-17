#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ranks = vec![5, 1, 8];
        let cars = 6;
        let ans = 16;
        assert_eq!(Solution::repair_cars(ranks, cars), ans);
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
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut l = 0;
        let mut r = i64::MAX;
        while l < r {
            let m = l + (r - l) / 2;
            if Self::check(&ranks, m) >= cars as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        r
    }

    fn check(ranks: &[i32], limit: i64) -> i64 {
        let mut ret = 0;
        for &r in ranks {
            ret += ((limit / r as i64) as f64).sqrt() as i64;
        }
        ret
    }
}
