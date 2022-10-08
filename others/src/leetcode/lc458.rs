#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let test_cnt = minutes_to_test / minutes_to_die;
        let mut ans = 0;
        while (test_cnt + 1).pow(ans) < buckets {
            ans += 1;
        }
        ans as i32
    }
}
