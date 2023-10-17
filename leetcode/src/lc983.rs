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
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut _days = vec![false; 366];
        for d in days {
            _days[d as usize] = true;
        }
        let days = _days;
        let mut dp = vec![i32::MAX; 366];
        dp[0] = 0;
        for i in 1..=365 {
            if days[i] {
                dp[i] = dp[i - 1] + costs[0];
                dp[i] = dp[i].min(if i >= 7 { dp[i - 7] } else { dp[0] } + costs[1]);
                dp[i] = dp[i].min(if i >= 30 { dp[i - 30] } else { dp[0] } + costs[2]);
            } else {
                dp[i] = dp[i - 1];
            }
        }
        dp[365]
    }
}
