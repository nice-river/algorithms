struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
		costs.sort_unstable();
		let mut ans = 0;
        for cost in costs {
            if coins >= cost {
				coins -= cost;
				ans += 1;
			} else {
				break;
			}
		}
        ans
    }
}