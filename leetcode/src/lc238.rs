#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 3, 4];
        let ans = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), ans);
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
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums
            .iter()
            .rev()
            .scan(1, |a, b| {
                *a = *a * *b;
                Some(*a)
            })
            .collect::<Vec<_>>();
        ans.reverse();
        let mut s = 1;
        for (i, num) in nums.into_iter().enumerate() {
            ans[i] = *ans.get(i + 1).unwrap_or(&1) * s;
            s *= num;
        }
        ans
    }
}
