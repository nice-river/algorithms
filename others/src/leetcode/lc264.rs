#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::nth_ugly_number(1), 1);
        assert_eq!(Solution::nth_ugly_number(2), 2);
        assert_eq!(Solution::nth_ugly_number(3), 3);
        assert_eq!(Solution::nth_ugly_number(4), 4);
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(11), 15);
        assert_eq!(Solution::nth_ugly_number(14), 20);
        assert_eq!(Solution::nth_ugly_number(15), 24);
        assert_eq!(Solution::nth_ugly_number(16), 25);
        assert_eq!(Solution::nth_ugly_number(18), 30);
        assert_eq!(Solution::nth_ugly_number(1000), 51200000);
        assert_eq!(Solution::nth_ugly_number(1690), 2123366400);
    }
}

struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_nums = vec![1; n as usize + 1];
        let (mut p2, mut p3, mut p5) = (1, 1, 1);
        for i in 2..=n as usize {
            let mini = (ugly_nums[p2] * 2).min(ugly_nums[p3] * 3).min(ugly_nums[p5] * 5);
            if mini == ugly_nums[p2] * 2 {
                p2 += 1;
            }
            if mini == ugly_nums[p3] * 3 {
                p3 += 1;
            }
            if mini == ugly_nums[p5] * 5 {
                p5 += 1;
            }
            ugly_nums[i] = mini;
        }
        ugly_nums[n as usize]
    }
}