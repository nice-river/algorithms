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
    pub fn binary_gap(n: i32) -> i32 {
        let b = format!("{n:b}");
        let b = b.as_bytes();
        let mut x = b.len();
        let mut ans = 0;
        for i in 0..b.len() {
            if b[i] == b'1' {
                if x != b.len() {
                    ans = ans.max(i - x);
                }
                x = i;
            }
        }
        ans as i32
    }
}
