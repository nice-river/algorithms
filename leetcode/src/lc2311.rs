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
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let t = format!("{k:b}").chars().collect::<Vec<_>>();
        if s.len() < t.len() {
            return s.len() as i32;
        }
        let mut zeros = 0;
        for i in 0..s.len() - t.len() {
            if s[i] == '0' {
                zeros += 1;
            }
        }
        for i in 0..t.len() {
            if t[i] > s[i + s.len() - t.len()] {
                return zeros + t.len() as i32;
            } else if t[i] < s[i + s.len() - t.len()] {
                return t.len() as i32 - 1 + zeros;
            }
        }
        return zeros + t.len() as i32;
    }
}
