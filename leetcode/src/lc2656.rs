#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let maxi = nums.into_iter().max().unwrap();
        (maxi + maxi + k - 1) * k / 2
    }
}
