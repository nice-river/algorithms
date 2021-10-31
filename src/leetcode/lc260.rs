#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |x, &y| x ^ y);
        let mut a = 0;
        for &num in nums.iter() {
            if (num & (xor & -xor)) != 0 {
                a ^= num;
            }
        }
        vec![a, xor ^ a]
    }
}
