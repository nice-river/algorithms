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
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 1;
        loop {
            if i * 2 < n {
                i *= 2;
            } else {
                i = (i - n / 2) * 2 + 1;
            }
            ans += 1;
            if i == 1 {
                break;
            }
        }
        ans
    }
}