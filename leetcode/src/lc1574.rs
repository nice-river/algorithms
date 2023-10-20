#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr = vec![5, 4, 3, 2, 1];
        let ans = 4;
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), ans);
    }

    #[test]
    fn test1() {
        let arr = vec![16, 10, 0, 3, 22, 1, 14, 7, 1, 12, 15];
        let ans = 8;
        assert_eq!(Solution::find_length_of_shortest_subarray(arr), ans);
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
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut q = arr.len() - 1;
        while q != 0 && arr[q - 1] <= arr[q] {
            q -= 1;
        }
        if q == 0 {
            return 0;
        }
        let mut ans = q as i32;
        for i in 0..q {
            if i != 0 && arr[i] < arr[i - 1] {
                break;
            }
            let mut l = q;
            let mut r = arr.len();
            while l < r {
                let m = (l + r) / 2;
                if arr[m] < arr[i] {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            ans = ans.min((r - i - 1) as i32);
        }
        ans
    }
}
