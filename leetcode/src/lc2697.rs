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

struct Solution {}

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut s = s.into_bytes();
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - 1 - i] {
                s[i] = s[i].min(s[n - 1 - i]);
                s[n - 1 - i] = s[i];
            }
        }
        String::from_utf8(s).unwrap()
    }
}
