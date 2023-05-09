#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test0() {
	}
}

struct Solution {}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
	    let w = (area as f64).sqrt() as i32;
	    for w in (1..=w).rev() {
		    if area % w == 0 {
			    return vec![area / w, w];
		    }
	    }
	    vec![area, 1]
    }
}