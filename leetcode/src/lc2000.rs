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
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut chars = word.chars().collect::<Vec<_>>();
        let mut p = chars.len();
        for i in 0..chars.len() {
            if chars[i] == ch {
                p = i;
                break;
            }
        }
        if p == chars.len() {
            chars.into_iter().collect()
        } else {
            for i in 0..=p / 2 {
                let c = chars[i];
                chars[i] = chars[p - i];
                chars[p - i] = c;
            }
            chars.into_iter().collect()
        }
    }
}
