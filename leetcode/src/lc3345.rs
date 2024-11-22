#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        for i in n.. {
            let mut k = i;
            let mut g = 1;
            while k != 0 {
                g *= k % 10;
                k /= 10;
            }
            if g % t == 0 {
                return i;
            }
        }
        unreachable!()
    }
}
