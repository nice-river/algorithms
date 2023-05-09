struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn lemonade_change(bills: Vec<i32>) -> bool {
		let mut map = [0; 21];
		for bill in bills.into_iter() {
			match bill {
				5 => {
					map[5] += 1;
				}
				10 => {
					if map[5] == 0 {
						return false;
					}
					map[10] += 1;
					map[5] -= 1;
				}
				20 => {
					if map[5] == 0 {
						return false;
					}
					if map[10] > 0 {
						map[10] -= 1;
						map[5] -= 1;
					} else {
						if map[5] >= 3 {
							map[5] -= 3;
						} else {
							return false;
						}
					}
				}
				_ => unreachable!()
			}
		}
		true
	}
}