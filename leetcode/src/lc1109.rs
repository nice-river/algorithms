struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
		let n = n as usize;
		let mut axies = vec![0; n + 1];
		let mut ans = vec![0; n];
		for booking in bookings {
			axies[booking[0] as usize - 1] += booking[2];
			axies[booking[1] as usize] -= booking[2];
		}
		ans[0] = axies[0];
		for i in 1..n {
			ans[i] = ans[i - 1] + axies[i];
		}
		ans
	}
}