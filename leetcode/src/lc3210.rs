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
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut ans = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            ans.push(s[(i + k as usize) % s.len()]);
        }

        String::from_utf8(ans).unwrap()
    }
}
