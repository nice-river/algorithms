#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let time = vec![1, 2, 3, 4];
        let total_trips = 25;
        let ans = 12;
        assert_eq!(Solution::minimum_time(time, total_trips), ans);
    }

    #[test]
    fn test1() {
        let time = vec![1, 2, 3];
        let total_trips = 5;
        let ans = 3;
        assert_eq!(Solution::minimum_time(time, total_trips), ans);
    }

    #[test]
    fn test2() {
        let time = vec![2];
        let total_trips = 1;
        let ans = 2;
        assert_eq!(Solution::minimum_time(time, total_trips), ans);
    }

    #[test]
    fn test3() {
        let time = vec![5, 10, 10];
        let total_trips = 9;
        let ans = 25;
        assert_eq!(Solution::minimum_time(time, total_trips), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut l = 0i64;
        let mut r = 2i64.pow(50);
        while l + 1 < r {
            let mut m = l + (r - l) / 2;
            let mut c = 0;
            for &t in &time {
                c += m / t as i64;
                if c >= total_trips as i64 {
                    break;
                }
            }
            if c >= total_trips as i64 {
                r = m;
            } else {
                l = m;
            }
        }
        r
    }
}
