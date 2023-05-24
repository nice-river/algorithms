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
    pub fn min_length(s: String) -> i32 {
        let mut s = s.into_bytes();
        let mut changed = true;
        while changed && s.len() >= 2 {
            changed = false;
            let mut k = s.len();
            for i in 0..s.len() - 1 {
                if &s[i..=i + 1] == b"AB" || &s[i..=i + 1] == b"CD" {
                    k = i;
                    break;
                }
            }
            if k != s.len() {
                s.remove(k + 1);
                s.remove(k);
                changed = true;
            }
        }
        s.len() as i32
    }
}
