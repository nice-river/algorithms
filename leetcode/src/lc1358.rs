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
    pub fn number_of_substrings(s: String) -> i32 {
        let mut ans = 0;
        let n = s.len();
        let mut nearest = vec![-1; 3];
        for (i, ch) in s.char_indices() {
            let p = ch as u8 - b'a';
            let mut k = n as i32;
            for j in 0..3 {
                if j != p {
                    k = k.min(nearest[j as usize]);
                }
            }
            if k != -1 {
                ans += k + 1;
            }
            nearest[p as usize] = i as i32;
        }
        ans
    }
}
