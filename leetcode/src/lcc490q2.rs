#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn is_digitorial_permutation(n: i32) -> bool {
        let mut p = n;
        let mut x = 0;
        let mut a = vec![0; 10];
        let mut b = vec![0; 10];
        while p != 0 {
            a[p as usize % 10] += 1;
            x += Self::calc_factorial(p % 10);
            p /= 10;
        }
        while x != 0 {
            b[x as usize % 10] += 1;
            x /= 10;
        }
        a == b
    }

    fn calc_factorial(n: i32) -> i32 {
        let mut ret = 1;
        for i in 1..=n {
            ret *= i;
        }
        ret
    }
}
