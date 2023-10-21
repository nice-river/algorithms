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

use crate::*;

struct Solution {}
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut arr = vec![a, b, c];
        arr.sort();
        let mut ans = vec![0, 0];
        ans[1] = arr[2] - arr[1] - 1 + arr[1] - arr[0] - 1;
        let g = arr[1] - arr[0] - 1;
        let k = arr[2] - arr[1] - 1;
        if g == 0 {
            if k > 0 {
                ans[0] = 1;
            }
        } else if g == 1 {
            ans[0] = 1;
        } else if k <= 1 {
            ans[0] = 1;
        } else {
            ans[0] = 2;
        }
        ans
    }
}
