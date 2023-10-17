#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3, 4, 1, 4];
        let ans = 2;
        assert_eq!(Solution::first_missing_positive(nums), ans);
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
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut ans = nums.len() as i32 + 1;
        for i in 0..nums.len() {
            while nums[i] > 0
                && nums[i] < ans
                && nums[i] - 1 != i as i32
                && nums[i] != nums[nums[i] as usize - 1]
            {
                let j = nums[i] as usize - 1;
                nums.swap(i, j);
            }
        }
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                ans = i as i32 + 1;
                break;
            }
        }
        ans
    }
}
