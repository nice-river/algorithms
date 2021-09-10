#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).map(|x| x as i32).unwrap_or(-1)
    }
}
