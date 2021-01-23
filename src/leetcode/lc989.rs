struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let a = vec![2,1,5];
		let k = 806;
		dbg!(Solution::add_to_array_form(a, k));
	}
}

impl Solution {
	pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
		let mut carry = k;
		let mut a = a.into_iter().rev().map(|v| {
			let k = carry + v;
			carry = k / 10;
			k % 10
		}).collect::<Vec<i32>>();
		while carry != 0 {
			a.push(carry % 10);
			carry /= 10;
		}
		a.reverse();
		a
	}
}