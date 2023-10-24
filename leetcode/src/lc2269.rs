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
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let s = num.to_string();
        let s = s.as_bytes();
        let mut ans = 0;
        for i in 0..s.len() {
            let j = i + k as usize;
            if j > s.len() {
                break;
            }
            let x = std::str::from_utf8(&s[i..j]).unwrap().parse::<i32>().unwrap();
            if x != 0 && num % x == 0 {
                ans += 1;
            }
        }
        ans
    }
}
