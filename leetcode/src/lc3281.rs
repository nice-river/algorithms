#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let start = vec![1000000000, 0];
        let d = 1000000000;
        let ans = 2000000000;
        assert_eq!(Solution::max_possible_score(start, d), ans);
    }

    #[test]
    fn test1() {
        let start = vec![6, 0, 3];
        let d = 2;
        let ans = 4;
        assert_eq!(Solution::max_possible_score(start, d), ans);
    }

    #[test]
    fn test2() {
        let start = vec![10, 0];
        let d = 0;
        let ans = 10;
        assert_eq!(Solution::max_possible_score(start, d), ans);
    }

    #[test]
    fn test3() {
        let start = vec![100, 1000000000, 0];
        let d = 1009;
        let ans = 1109;
        assert_eq!(Solution::max_possible_score(start, d), ans);
    }

    #[test]
    fn test4() {
        let start = vec![10, 7, 5, 0];
        let d = 0;
        let ans = 2;
        assert_eq!(Solution::max_possible_score(start, d), ans);
    }

}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
        let mut arr = start
            .into_iter()
            .map(|x| (x as i64, (x + d) as i64))
            .collect::<Vec<_>>();
        arr.sort();
        let mut l = 0;
        let mut r = arr.last().unwrap().1 - arr[0].0 + 1;
        while l + 1 < r {
            let m = l + (r - l) / 2;
            let mut a = arr[0].0;
            let mut passed = true;
            for &(x, y) in arr.iter().skip(1) {
		if y < a + m {
		    passed = false;
		    break;
		}
		a = x.max(a + m);
            }
            if passed {
                l = m;
            } else {
                r = m;
            }
        }
        l as i32
    }
}
