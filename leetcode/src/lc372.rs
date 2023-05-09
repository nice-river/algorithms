#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = (2i64.pow(31) - 1) as i32;
        let b = vec![0];
        let ans = 1;
        assert_eq!(Solution::super_pow(a, b), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const MOD: i32 = 1337;
        let mut ans = 1;
        let mut base = a % MOD;
        for &num in b.iter().rev() {
            for _ in 0..num {
                ans = ans * base % MOD;
            }
            base = (0..10).fold(1, |k, _| k * base % MOD);
        }
        ans
    }
}
