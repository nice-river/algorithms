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
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut ones = 0;
        let mut zeros = 0;
        for ch in s.chars() {
            if ch == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        format!("{}{}1", "1".repeat(ones - 1), "0".repeat(zeros))
    }
     
}
