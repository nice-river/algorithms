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
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                let wi = words[i].as_bytes();
                let wj = words[j].as_bytes();
                if wi.len() == wj.len() {
                    if wi.iter().zip(wj.iter().rev()).all(|(a, b)| a == b) {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
