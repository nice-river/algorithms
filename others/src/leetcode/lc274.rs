#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
	    let n = citations.len();
	    let mut l = 0 as usize;
	    let mut r = n + 1;
	    while l + 1 < r {
		    let m = l + (r - l) / 2;
		    let cnt = citations.iter().filter(|&&citation| citation >= m as i32).count();
		    if cnt >= m {
			    l = m;
		    } else {
			    r = m;
		    }
	    }
	    l as i32
    }
}