#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 1;
        let presses = 0;
        let ans = 1;
        assert_eq!(Solution::flip_lights(n, presses), ans);
    }
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let arr = vec![1; n as usize];
        let mut rs = HashSet::new();
        if presses % 2 == 0 {
            rs.insert(arr.clone());
            if presses >= 2 {
                rs.insert(Solution::flip_0(Solution::flip_1(arr.clone())));
                rs.insert(Solution::flip_0(Solution::flip_2(arr.clone())));
                rs.insert(Solution::flip_0(Solution::flip_3(arr.clone())));
                rs.insert(Solution::flip_1(Solution::flip_2(arr.clone())));
                rs.insert(Solution::flip_1(Solution::flip_3(arr.clone())));
                rs.insert(Solution::flip_2(Solution::flip_3(arr.clone())));
            }
            if presses >= 4 {
                rs.insert(Solution::flip_0(Solution::flip_1(Solution::flip_2(
                    Solution::flip_3(arr.clone()),
                ))));
            }
        } else {
            rs.insert(Solution::flip_0(arr.clone()));
            rs.insert(Solution::flip_1(arr.clone()));
            rs.insert(Solution::flip_2(arr.clone()));
            rs.insert(Solution::flip_3(arr.clone()));
            if presses >= 3 {
                rs.insert(Solution::flip_0(Solution::flip_1(Solution::flip_2(
                    arr.clone(),
                ))));
                rs.insert(Solution::flip_0(Solution::flip_1(Solution::flip_3(
                    arr.clone(),
                ))));
                rs.insert(Solution::flip_0(Solution::flip_2(Solution::flip_3(
                    arr.clone(),
                ))));
                rs.insert(Solution::flip_1(Solution::flip_2(Solution::flip_3(
                    arr.clone(),
                ))));
            }
        }
        rs.len() as i32
    }

    fn flip_0(mut arr: Vec<u8>) -> Vec<u8> {
        for x in &mut arr {
            *x = !*x;
        }
        arr
    }

    fn flip_1(mut arr: Vec<u8>) -> Vec<u8> {
        for x in arr.iter_mut().step_by(2) {
            *x = !*x;
        }
        arr
    }

    fn flip_2(mut arr: Vec<u8>) -> Vec<u8> {
        for x in arr.iter_mut().skip(1).step_by(2) {
            *x = !*x;
        }
        arr
    }

    fn flip_3(mut arr: Vec<u8>) -> Vec<u8> {
        for x in arr.iter_mut().step_by(3) {
            *x = !*x;
        }
        arr
    }
}
