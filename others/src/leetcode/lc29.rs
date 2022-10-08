#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let dividend = 1928374934;
        let divisor = -3892;
        let ans = -495471;
        assert_eq!(Solution::divide(dividend, divisor), ans);
    }

    #[test]
    fn test1() {
        let dividend = 10;
        let divisor = 2;
        let ans = 5;
        assert_eq!(Solution::divide(dividend, divisor), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let is_rev = dividend < 0 && divisor > 0 || dividend > 0 && divisor < 0;
        if dividend > 0 {
            dividend = -dividend;
        }
        if divisor > 0 {
            divisor = -divisor;
        }
        let mut arr = vec![divisor];
        loop {
            let v = *arr.last().unwrap();
            if v >= dividend - v {
                arr.push(v + v);
            } else {
                break;
            }
        }
        let mut ans = 0;
        for (i, p) in (arr.into_iter().enumerate().rev()) {
            if p >= dividend {
                ans += (1 << i);
                dividend -= p;
            }
        }
        if is_rev {
            -ans
        } else {
            ans
        }
    }
}
