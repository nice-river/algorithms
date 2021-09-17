#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2];
        let ans = 1;
        assert_eq!(Solution::find_peak_element(nums), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![1];
        let ans = 0;
        assert_eq!(Solution::find_peak_element(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 1];
        let ans = 0;
        assert_eq!(Solution::find_peak_element(nums), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3, 2, 1];
        let ans = 2;
        assert_eq!(Solution::find_peak_element(nums), ans);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 2, 3, 4, 1];
        let ans = 3;
        assert_eq!(Solution::find_peak_element(nums), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l + 1 < r {
            let m = (r + l) / 2;
            if m > 0 {
                if nums[m] < nums[m - 1] {
                    r = m;
                } else {
                    l = m;
                }
            } else {
                if nums[m] < nums[m + 1] {
                    l = m + 1;
                } else {
                    r = m + 1;
                }
            }
        }
        l as i32
    }
}
