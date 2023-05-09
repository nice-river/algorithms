#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr = vec![1, 2, 1];
        let ans = 1;
        assert_eq!(Solution::peak_index_in_mountain_array(arr), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut l = 1;
        let mut r = arr.len() - 1;
        while l + 1 < r {
            let m = (l + r) / 2;
            if arr[m] < arr[m - 1] {
                r = m;
            } else {
                l = m;
            }
        }
        l as i32
    }
}
