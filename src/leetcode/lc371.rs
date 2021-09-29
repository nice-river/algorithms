#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = 1;
        let b = 2;
        let ans = 3;
        assert_eq!(Solution::get_sum(a, b), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let c = a;
            a ^= b;
            b &= c;
            b <<= 1;
        }
        a
    }
}
