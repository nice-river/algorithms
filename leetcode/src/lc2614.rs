#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            if Self::is_prime(nums[i][i]) {
                ans = ans.max(nums[i][i]);
            }
            if Self::is_prime(nums[i][n - 1 - i]) {
                ans = ans.max(nums[i][n - 1 - i]);
            }
        }
        ans
    }

    fn is_prime(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        if num == 2 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }
        for i in (3..=(num as f64).sqrt() as i32).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }
}
