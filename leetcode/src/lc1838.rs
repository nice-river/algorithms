struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let nums = vec![1, 2, 4];
        let k = 5;
        let ans = 3;
        assert_eq!(Solution::max_frequency(nums, k), ans);
	}

    #[test]
    fn test1() {
        let nums = vec![1,4,8,13];
        let k = 5;
        let ans = 2;
        assert_eq!(Solution::max_frequency(nums, k), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![3,9,6];
        let k = 2;
        let ans = 1;
        assert_eq!(Solution::max_frequency(nums, k), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 2, 100];
        let k = 2;
        let ans = 3;
        assert_eq!(Solution::max_frequency(nums, k), ans);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 1, 1, 2, 50, 50, 100];
        let k = 99;
        let ans = 4;
        assert_eq!(Solution::max_frequency(nums, k), ans);
    }
}


impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 0 { return 0; }
        nums.sort_unstable();
        let mut l = 0;
        let mut r = 1;
        let mut ans = 1;
        let mut s = nums[0] as usize;
        loop {
            while r < nums.len() && (s + (nums[r] + k) as usize ) >= (r - l + 1) as usize * nums[r] as usize {
                s += nums[r] as usize;
                r += 1;
            }
            ans = ans.max(r - l);
            if r == nums.len() {
                break;
            }
            while l < r && (s + (nums[r] + k) as usize) < (r - l + 1) as usize * nums[r] as usize {
                s -= nums[l] as usize;
                l += 1;
            }
        }
        ans as i32
    }
}