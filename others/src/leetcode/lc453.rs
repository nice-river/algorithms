#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let mut s = nums[0] as i64;
        let mut mini = nums[0] as i64;
        for &num in nums.iter().skip(1) {
            mini = mini.min(num as i64);
            s += num as i64;
        }
        let k = mini * (1 - n) + s;
        (k - mini) as i32
    }
}
