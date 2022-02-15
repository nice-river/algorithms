#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num = 6;
        assert!(Solution::check_perfect_number(num));
    }
}

struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut x = 0;
        for i in 2..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                x += i;
                if num / i != i {
                    x += num / i;
                }
            }
        }
        x == num - 1
    }
}
