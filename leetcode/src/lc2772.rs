#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 2, 3, 1, 1, 0];
        let k = 3;
        assert!(Solution::check_array(nums, k))
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
    pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut op = vec![0; n + 1];
        let mut val = 0;
        for i in 0..n + 1 - k as usize {
            val += op[i];
            if nums[i] < val {
                return false;
            }
            op[i + k as usize] -= nums[i] - val;
            val = nums[i];
        }
        for i in n + 1 - k as usize..n {
            val += op[i];
            if nums[i] != val {
                return false;
            }
        }
        true
    }
}
