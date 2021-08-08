#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
	    let mut t = vec![0, 1, 1];
	    if n < 3 {
		    return t[n as usize];
	    }
	    for _ in 3..=n {
		    let x = t.iter().sum();
		    t[0] = t[1];
		    t[1] = t[2];
		    t[2] = x;
	    }
	    t[2]
    }
}