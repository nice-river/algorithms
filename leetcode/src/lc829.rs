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
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=(((n * 2) as f64).sqrt() as i32) {
            let k = n - i * (i - 1) / 2;
            if k % i == 0 {
                ans += 1;
            }
        }
        ans
    }
}