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
    pub fn count_substrings(s: String, c: char) -> i64 {
        let count = s.chars().filter(|x| x == &c).count();
        let mut ans = count as i64;
        ans += ans * (ans - 1) / 2;
        ans
    }
}
