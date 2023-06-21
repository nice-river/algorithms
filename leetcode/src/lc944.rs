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
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut ans = 0;
        for i in 0..strs[0].len() {
            for j in 1..strs.len() {
                let a = strs[j - 1].as_bytes();
                let b = strs[j].as_bytes();
                if a[i] > b[i] {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}
