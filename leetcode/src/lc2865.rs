#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let max_heights = vec![5, 3, 4, 1, 1];
        let ans = 13;
        assert_eq!(Solution::maximum_sum_of_heights(max_heights), ans);
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut a = vec![0; max_heights.len()];
        let mut stk = vec![];
        a[0] = max_heights[0] as i64;
        stk.push((max_heights[0], 0));
        for i in 1..max_heights.len() {
            while !stk.is_empty() && stk.last().unwrap().0 >= max_heights[i] {
                stk.pop();
            }
            if let Some(&(h, p)) = stk.last() {
                a[i] = a[p] + max_heights[i] as i64 * (i as usize - p) as i64;
            } else {
                a[i] = max_heights[i] as i64 * (i as i64 + 1);
            }
            stk.push((max_heights[i], i));
        }
        let n = max_heights.len();
        let mut b = vec![0; max_heights.len()];
        stk.clear();
        b[n - 1] = max_heights[n - 1] as i64;
        stk.push((max_heights[n - 1], n - 1));
        for i in (0..max_heights.len() - 1).rev() {
            while !stk.is_empty() && stk.last().unwrap().0 >= max_heights[i] {
                stk.pop();
            }
            if let Some(&(h, p)) = stk.last() {
                b[i] = b[p] + max_heights[i] as i64 * (p - i as usize) as i64;
            } else {
                b[i] = max_heights[i] as i64 * (n - i) as i64;
            }
            stk.push((max_heights[i], i));
        }
        for i in 0..n {
            ans = ans.max(a[i] + b[i] - max_heights[i] as i64);
        }
        ans
    }
}
