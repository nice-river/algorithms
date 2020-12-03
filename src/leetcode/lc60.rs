struct Solution {}

impl Solution {
	pub fn get_permutation(n: i32, k: i32) -> String {
		let mut fac = vec![0; n as usize + 1];
		fac[0] = 1i32;
		for i in 1..fac.len() {
			fac[i] = fac[i - 1] * i as i32;
		}
		let mut ans = Vec::new();
		Solution::dfs(&mut ans, k - 1, &fac, &mut (1..n+1).collect::<Vec<i32>>());

		ans.into_iter().map(|v| v.to_string()).collect::<String>()
	}

	fn dfs(ans: &mut Vec<i32>, k: i32, fac: &Vec<i32>, arr: &mut Vec<i32>) {
		if ans.len() + 1 == fac.len() {
			return ;
		}
		let mut tot = 0;
		for i in 0..arr.len() {
			if tot + fac[fac.len() - ans.len() - 2] > k {
				ans.push(arr[i]);
				arr.remove(i);
				Solution::dfs(ans, k - tot, fac, arr);
				break;
			}
			tot += fac[fac.len() - ans.len() - 2];
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::get_permutation(2, 2), String::from("21"));
		assert_eq!(Solution::get_permutation(3, 3), String::from("213"));
		assert_eq!(Solution::get_permutation(4, 9), String::from("2314"));
	}
}
