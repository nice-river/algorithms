#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 1;
        let ans = true;
        assert_eq!(Solution::is_power_of_three(n), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n >= 1 && n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}
