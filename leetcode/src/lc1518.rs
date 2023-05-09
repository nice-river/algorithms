#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num_bottles = 9;
        let num_exchange = 3;
        let ans = 13;
        assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = 0;
        let mut empty_bottles = 0;
        while num_bottles != 0 {
            ans += num_bottles;
            let k = num_bottles + empty_bottles;
            num_bottles = k / num_exchange;
            empty_bottles = k % num_exchange
        }
        ans
    }
}
