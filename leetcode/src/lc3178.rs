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
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let mut p = 0;
        let mut op  = 1;
        for _ in 0..k {
            p += op;
            if p == n - 1 {
                op = -1;
            } else if p == 0 {
                op = 1;
            }
        }
        p
    }
}
