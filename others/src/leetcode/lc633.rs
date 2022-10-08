struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let c = 2147483647;
	}
}


impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let x = ((c / 2) as f64).sqrt() as i32;
		for i in 0..=x {
            let y2 = c - i * i;
            let y = (y2 as f64).sqrt() as i32;
			if y * y == y2 {
				return true;
			}
		}
        false
	}
}