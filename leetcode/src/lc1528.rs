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
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let s = s.into_bytes();
        let mut t = s.clone();
        for (i, c) in s.into_iter().enumerate() {
            t[indices[i] as usize] = c;
        }
        String::from_utf8(t).unwrap()
    }
}
