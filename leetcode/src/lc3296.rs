#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mountain_height = 4;
        let worker_times = vec![2, 1, 1];
        let ans = 3;
        assert_eq!(
            Solution::min_number_of_seconds(mountain_height, worker_times),
            ans
        );
    }

    #[test]
    fn test1() {
        let mountain_height = 10;
        let worker_times = vec![3, 2, 2, 4];
        let ans = 12;
        assert_eq!(
            Solution::min_number_of_seconds(mountain_height, worker_times),
            ans
        );
    }

    #[test]
    fn test2() {
        let mountain_height = 5;
        let worker_times = vec![1];
        let ans = 15;
        assert_eq!(
            Solution::min_number_of_seconds(mountain_height, worker_times),
            ans
        );
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut left = 1;
        let mut right = i64::MAX;
        let mountain_height = mountain_height as i64;
        while left < right {
            let mid = left + (right - left) / 2;
            let mut k = 0;
            for &t in &worker_times {
                let t = mid / t as i64;
                k += Self::days(t);
                if k >= mountain_height {
                    break;
                }
            }
            if k >= mountain_height {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        right
    }

    fn days(cnt: i64) -> i64 {
        let mut g = ((cnt as f64).sqrt() * 2f64.sqrt()) as i64;
        loop {
            if g % 2 == 0 {
                if let Some(x) = (g / 2).checked_mul(g + 1) {
                    if x > cnt {
                        g -= 1;
                    } else {
                        break;
                    }
                } else {
                    g -= 1;
                }
            } else if let Some(x) = ((g + 1) / 2).checked_mul(g) {
                if x > cnt {
                    g -= 1;
                } else {
                    break;
                }
            } else {
                g -= 1;
            }
        }
        g
    }
}
