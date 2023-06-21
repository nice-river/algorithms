#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2];
        let ans = vec![2, 1];
        assert_eq!(Solution::smallest_subarrays(nums), ans);
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
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        const N: usize = 32;
        let n = nums.len();
        let mut h = vec![vec![nums.len(); N]; n];
        for j in 0..N {
            if (nums[n - 1] & (1 << j)) != 0 {
                h[n - 1][j] = n - 1;
            }
        }
        for i in (0..n - 1).rev() {
            for j in 0..N {
                if (nums[i] & (1 << j)) != 0 {
                    h[i][j] = i;
                } else {
                    h[i][j] = h[i + 1][j];
                }
            }
        }
        let mut ans = vec![];
        for i in 0..n {
            if let Some(&t) = h[i].iter().filter(|&&x| x != n).max() {
                ans.push((t - i + 1) as i32);
            } else {
                ans.push(1);
            }
        }
        ans
    }
}
