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
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let t = t.as_bytes();
        let mut ans = 0;
        for (i, c) in s.as_bytes().iter().enumerate() {
            let p = t.iter().position(|x| x == c).unwrap();
            ans += (i as i32 - p as i32).abs();
        }
        ans
    }
}
