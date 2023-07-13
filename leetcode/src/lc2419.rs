#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let maxi = *nums.iter().max().unwrap();
        let mut longest = 0;
        for i in 0..nums.len() {
            if nums[i] == maxi {
                longest += 1;
            } else {
                ans = ans.max(longest);
                longest = 0;
            }
        }
        ans.max(longest)
    }
}
