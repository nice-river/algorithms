#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
    }

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
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        if stations.len() == 0 {
            return if start_fuel >= target { 0 } else { -1 };
        }
        let n = stations.len();
        let mut dp = vec![vec![-1i64; n + 1]; n];
        if stations[0][0] <= start_fuel {
            dp[0][0] = start_fuel as i64 - stations[0][0] as i64;
            dp[0][1] = start_fuel as i64 - stations[0][0] as i64 + stations[0][1] as i64;
        } else {
            return -1;
        }
        for i in 1..stations.len() {
            let pos = stations[i][0] as i64;
            let fuel = stations[i][1] as i64;
            let dist = pos - stations[i - 1][0] as i64;
            for j in 0..=n {
                if dp[i - 1][j] >= dist {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j] - dist);
                }
                if j > 0 && dp[i - 1][j - 1] >= dist {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] - dist + fuel);
                }
            }
        }
        for i in 0..=n {
            if dp[n - 1][i] >= target as i64 - stations[n - 1][0] as i64 {
                return i as i32;
            }
        }
        -1
    }
}
