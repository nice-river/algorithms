#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1];
        let ans = vec![1];
        assert_eq!(Solution::sort_even_odd(nums), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn sort_even_odd(mut nums: Vec<i32>) -> Vec<i32> {
        let mut evens = nums.iter().step_by(2).copied().collect::<Vec<i32>>();
        let mut odds = nums
            .iter()
            .skip(1)
            .step_by(2)
            .copied()
            .collect::<Vec<i32>>();
        evens.sort_unstable_by_key(|k| -k);
        odds.sort_unstable();
        let mut ans = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            if i % 2 == 0 {
                ans.push(evens.pop().unwrap());
            } else {
                ans.push(odds.pop().unwrap());
            }
        }
        ans
    }
}
