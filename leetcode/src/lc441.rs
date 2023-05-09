#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test0() {
	}
}

struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let n = n as i64;
        let mut l = 0;
        let mut r = 2i64.pow(20);
        while l + 1 < r {
            let m = (l + r) / 2;
            if m * (m + 1) / 2 <= n {
                l = m;
            } else {
                r = m;
            }
        }
        l as i32
    }
}