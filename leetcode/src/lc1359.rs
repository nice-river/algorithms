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
    pub fn count_orders(n: i32) -> i32 {
        const MODULE: i64 = 1_000_000_007;
        let mut ans = 1i64;
        for i in 1..n as i64 {
            let k = (i * 2) + 1;
            let g = (k + 1) * k / 2;
            ans = (ans * g) % MODULE;
        }
        ans as i32
    }
}
