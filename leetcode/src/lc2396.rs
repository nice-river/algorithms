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
    pub fn is_strictly_palindromic(n: i32) -> bool {
        let mut arr = vec![];
        'outer: for i in 2..=n - 2 {
            arr.clear();
            let mut x = n;
            while x != 0 {
                arr.push(x % i);
                x /= i;
            }
            for j in 0..arr.len() / 2 {
                if arr[j] != arr[arr.len() - 1 - j] {
                    return false;
                }
            }
        }
        true
    }
}
