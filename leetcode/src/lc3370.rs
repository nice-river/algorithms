#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn smallest_number(mut n: i32) -> i32 {
	let mut ans = 0;
	while n > 0 {
	    ans = ans * 2 + 1;
	    n /= 2;
	}
	ans
    }
}
