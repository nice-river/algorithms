#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

struct Solution {}

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 0;
        }
        let mut dp = vec![0; n + 1];
        for i in 2..=n {
            dp[i] = i;
        }
        for i in 2..n {
            let mut k = 1;
            while i + k * i <= n {
                dp[i + k * i] = dp[i + k * i].min(dp[i] + 1 + k);
                k += 1;
            }
        }
        dp[n] as i32
    }
}
