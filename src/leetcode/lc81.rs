struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![3,5,1];
        assert_eq!(Solution::search(arr, 3), true);

        let arr = vec![1,2,0,1,1,1];
        assert_eq!(Solution::search(arr, 0), true);

        let arr = vec![1,0,1,1,1];
        assert_eq!(Solution::search(arr, 0), true);
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] == target {
                return true;
            } else if nums[m] < target {
                if nums[l] < nums[r-1] {
                    l = m + 1;
                } else if nums[l] > nums[r-1] {
                    if nums[l] > target {
                        l = m + 1;
                    } else if nums[l] < target {
                        if nums[l] == nums[m] {
                            l += 1;
                        } else if nums[l] < nums[m] {
                            l = m + 1;
                        } else {
                            r = m;
                        }
                    } else {
                        return true;
                    }
                } else {
                    l += 1;
                }
            } else {
                if nums[l] < nums[r-1] {
                    r = m
                } else if nums[l] > nums[r-1] {
                    if nums[l] < target {
                        r = m;
                    } else if nums[l] > target {
                        if nums[l] == nums[m] {
                            l += 1;
                        } else if nums[l] < nums[m] {
                            l = m + 1;
                        } else {
                            r = m;
                        }
                    } else {
                        return true;
                    }
                } else {
                    l += 1;
                }
            }
        }
        false
    }
}