#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3, 2, 1];
        let ans = 1;
        assert_eq!(Solution::third_max(nums), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut maxis = vec![];
        for _ in 0..3 {
            let mut k = None;
            for &num in nums.iter() {
                if !maxis.contains(&num) {
                    if let Some(x) = k {
                        if num > x {
                            k = Some(num);
                        }
                    } else {
                        k = Some(num);
                    }
                }
            }
            if let Some(x) = k {
                maxis.push(x);
            }
        }
        if maxis.len() < 3 {
            maxis[0]
        } else {
            maxis[2]
        }
    }
}
