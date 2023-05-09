#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let tires = vec![vec![2, 3], vec![3, 4]];
        let change_time = 5;
        let num_laps = 4;
        let ans = 21;
        assert_eq!(
            Solution::minimum_finish_time(tires, change_time, num_laps),
            ans
        );
    }

    #[test]
    fn test1() {
        let tires = vec![vec![1, 10], vec![2, 2], vec![3, 4]];
        let change_time = 6;
        let num_laps = 5;
        let ans = 25;
        assert_eq!(
            Solution::minimum_finish_time(tires, change_time, num_laps),
            ans
        );
    }
}

struct Solution {}

impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let num_laps = num_laps as usize;
        let mut dp = vec![i64::MAX; num_laps + 1];
        dp[0] = 0;
        for tire in &tires {
            let mut s = 0i64;
            let mut p = tire[0] as i64;
            for i in 1..=num_laps {
                if let Some(x) = s.checked_add(p) {
                    dp[i] = dp[i].min(x);
                    s += p;
                    if let Some(x) = p.checked_mul(tire[1] as i64) {
                        p = x;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        for i in 2..=num_laps {
            for j in 1..=i / 2 {
                dp[i] = dp[i].min(dp[j] + change_time as i64 + dp[i - j])
            }
        }
        dp[num_laps] as i32
    }
}
