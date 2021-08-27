use std::cmp::Ordering;

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(unsafe {Solution::guessNumber(10)}, 6);
	}
}

struct Solution {}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
	    let mut l = 1;
	    let mut r = n;
	    loop {
		    let m = l + (r - l) / 2;
		    match guess(m) {
			    -1 => r = m - 1,
			    0 => return m,
			    1 => l = m + 1,
			    _ => unreachable!()
		    }
	    }
    }
}

unsafe fn guess(num: i32) -> i32 {
	0
}