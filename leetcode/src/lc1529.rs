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
    pub fn min_flips(target: String) -> i32 {
        let mut ans = 0;
        let mut t = 0;
        for &c in target.as_bytes() {
            let c = c - b'0';
            if c != t {
                ans += 1;
                t ^= 1;
            }
        }
        ans
    }
}
