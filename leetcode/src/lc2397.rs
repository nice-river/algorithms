#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let matrix = to_vec2d([[1], [0]]);
        let num_select = 1;
        let ans = 2;
        assert_eq!(Solution::maximum_rows(matrix, num_select), ans);
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
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut nums = vec![0; matrix.len()];
        for i in 0..matrix.len() {
            for (j, &b) in matrix[i].iter().enumerate() {
                nums[i] |= b << j;
            }
        }
        let mut ans = 0;
        for i in 0..1 << m {
            let mut s = 0;
            let mut k = i;
            while k != 0 {
                if k % 2 == 1 {
                    s += 1;
                }
                k /= 2;
            }
            if s == num_select {
                let mut t = 0;
                for &num in &nums {
                    if (num & i) ^ num == 0 {
                        t += 1;
                    }
                }
                ans = ans.max(t);
            }
        }
        ans
    }
}
