struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "2820022842865610841740282445647388119521934031292".to_string();
		assert_eq!(Solution::split_into_fibonacci(s), vec![28,200,228,428,656,1084,1740,2824,4564,7388,11952,19340,31292]);
	}
}


impl Solution {
	pub fn split_into_fibonacci(s: String) -> Vec<i32> {
		let s: Vec<u8> = s.into_bytes().iter().map(|&e| e - b'0').collect();
		let mut ans = vec![];
		for i in 1..s.len() {
			if s[0] == 0 && i > 1 {
				break;
			}
			for j in i+1..s.len() {
				if s[i] == 0 && j - i > 1 {
					break;
				}
				ans.clear();
				let a = Solution::to_num(&s[..i]);
				if a == -1 {
					break;
				}
				let b = Solution::to_num(&s[i..j]);
				if b == -1 {
					break;
				}
				ans.push(a);
				ans.push(b);
				if Solution::dfs(a, b, j, &s, &mut ans) {
					return ans;
				}
			}
		}
		vec![]
	}

	fn dfs(a: i32, b: i32, idx: usize, arr: &Vec<u8>, ans: &mut Vec<i32>) -> bool {
		if idx == arr.len() {
			return true;
		}

		if a.checked_add(b).is_none() {
			return false;
		}

		let c = (a + b).to_string().into_bytes().iter().map(|&e| e - b'0').collect::<Vec<u8>>();
		let cl = c.len();
		if arr.len() < cl + idx {
			return false;
		}

		for i in 0..cl {
			if c[i] != arr[idx + i] {
				return false;
			}
		}

		let c = Solution::to_num(&c);
		ans.push(c);
		Solution::dfs(b, c, idx + cl,  arr, ans)
	}


	fn to_num(s: &[u8]) -> i32 {
		let mut ret = 0i32;
		for &e in s {
			if ret.checked_mul(10).is_none() {
				return -1;
			}
			ret *= 10;
			if ret.checked_add(e as i32).is_none() {
				return -1;
			}
			ret += e as i32;
		}
		ret
	}
}