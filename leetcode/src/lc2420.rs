#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 1, 1, 1, 3, 4, 1];
        let k = 2;
        let ans = vec![2, 3];
        assert_eq!(Solution::good_indices(nums, k), ans);
    }

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
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        let mut decs = vec![1; nums.len()];
        let mut incs = vec![1; nums.len()];
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                decs[i] = decs[i - 1] + 1;
            }
        }
        for i in (0..nums.len() - 1).rev() {
            if nums[i] <= nums[i + 1] {
                incs[i] = incs[i + 1] + 1;
            }
        }
        let mut ans = vec![];
        for i in 1..nums.len() - 1 {
            if decs[i - 1] >= k && incs[i + 1] >= k {
                ans.push(i as i32);
            }
        }
        ans
    }
}
