#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let time_series = vec![1, 2];
        let duration = 2;
        let ans = 3;
        assert_eq!(Solution::find_poisoned_duration(time_series, duration), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut ans = 0;
        let mut last_t = -1;
        for t in time_series {
            let k = t + duration - 1;
            ans += last_t.max(k) - last_t.max(t - 1);
            last_t = last_t.max(k);
        }
        ans
    }
}
