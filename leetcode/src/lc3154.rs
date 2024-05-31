#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let k = 536870897;
        let ans = 155117520;
        assert_eq!(Solution::ways_to_reach_stair(k), ans);
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
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        let mut t = 0;
        let mut v = 1;
        let mut ans = 0;
        loop {
            if v - (t + 1) <= k {
                if v >= k {
                    ans += Self::count(t + 1, v - k);
                }
            } else {
                break;
            }
            v += 1 << t;
            t += 1;
        }
        ans
    }

    fn count(a: i32, b: i32) -> i32 {
        if a < b {
            return 0;
        }
        let mut r = 1;
        let mut y = 2;
        for x in (b + 1)..=a {
            r *= x as i64;
            if y <= (a - b) && r % y as i64 == 0 {
                r /= y as i64 ;
                y += 1;
            }
        }
        r as i32
    }
}
