#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 3];
        let ans = vec![];
        assert_eq!(Solution::majority_element(nums), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![1, 1, 3];
        let ans = vec![1];
        assert_eq!(Solution::majority_element(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 1, 1, 3, 3, 2, 2, 2];
        let ans = vec![1, 2];
        assert_eq!(Solution::majority_element(nums), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![2, 2];
        let ans = vec![2];
        assert_eq!(Solution::majority_element(nums), ans);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 1, 1, 2, 3, 7, 8, 1, 6, 9];
        let ans = vec![1];
        assert_eq!(Solution::majority_element(nums), ans);
    }

    #[test]
    fn test5() {
        let nums = vec![0, -1, 2, -1];
        let ans = vec![-1];
        assert_eq!(Solution::majority_element(nums), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> Vec<i32> {
        let mut elems = [0; 2];
        let mut votes = [0; 2];
        for &num in nums.iter() {
            if votes[0] > 0 && elems[0] == num {
                votes[0] += 1;
            } else if votes[1] > 0 && elems[1] == num {
                votes[1] += 1;
            } else if votes[0] == 0 {
                elems[0] = num;
                votes[0] = 1;
            } else if votes[1] == 0 {
                elems[1] = num;
                votes[1] = 1;
            } else {
                votes[0] -= 1;
                votes[1] -= 1;
            }
        }
        let mut ans = vec![];
        for i in 0..2 {
            if votes[i] > 0 && nums.iter().filter(|&num| num == &elems[i]).count() > nums.len() / 3
            {
                ans.push(elems[i]);
            }
        }
        ans
    }
}
