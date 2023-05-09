#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test0() {
		let num = "123".to_string();
		let target = 6;
		let ans = vec!["1+2+3".to_string(), "1*2*3".to_string()];
		assert_eq!(Solution::add_operators(num, target), ans);
	}

	#[test]
	fn test1() {
		let num = "12".to_string();
		let target = 2;
		let ans = vec!["1*2".to_string()];
		assert_eq!(Solution::add_operators(num, target), ans);
	}

	#[test]
	fn test2() {
		let num = "6".to_string();
		let target = 6;
		let ans : Vec<String> = vec![];
		assert_eq!(Solution::add_operators(num, target), ans);
	}
}

struct Solution {}

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
	    let mut ans = vec![];
	    let mut expr = (&num[..1]).to_string();
	    Solution::dfs(1, num.as_bytes(), target, &mut expr, &mut ans);
	    ans
    }

	fn dfs(idx: usize, num: &[u8], target: i32, expr: &mut String, ans: &mut Vec<String>) {
		if idx == num.len() {
			if Solution::is_eq_target(expr.as_bytes(), target) {
				ans.push(expr.clone());
			}
			return ;
		}
		for &op in ['+', '-', '*'].into_iter() {
			expr.push(op);
			expr.push(num[idx] as char);
			Solution::dfs(idx + 1, num, target, expr, ans);
			expr.pop();
			expr.pop();
		}
		expr.push(num[idx] as char);
		Solution::dfs(idx + 1, num, target, expr, ans);
		expr.pop();
	}

	fn is_eq_target(expr: &[u8], target: i32) -> bool {
		if let Some(x) = Solution::compute_expr(expr) {
			x == target as i64
		} else {
			false
		}
	}

	fn compute_expr(expr: &[u8]) -> Option<i64> {
		let mut nums = vec![];
		let mut ops = vec![];
		let mut i = 0;
		while i < expr.len() {
			if expr[i] == b'0' && i + 1 < expr.len() && Solution::is_digit(expr[i + 1]) {
				return None;
			}
			let mut j = i;
			while j < expr.len() && Solution::is_digit(expr[j]) {
				j += 1;
			}
			let num = std::str::from_utf8(&expr[i..j]).unwrap().parse::<i64>().unwrap();
			nums.push(num);
			if j < expr.len() {
				ops.push(expr[j]);
				j += 1;
			}
			i = j;
		}
		let mut p = vec![nums[0]];
		let mut q = vec![];
		for i in 0..ops.len() {
			if ops[i] == b'*' {
				let x = p.pop().unwrap();
				p.push(x * nums[i + 1]);
			} else {
				p.push(nums[i + 1]);
				q.push(ops[i]);
			}
		}
		let mut ret = p[0];
		for i in 0..q.len() {
			if q[i] == b'+' {
				ret += p[i + 1];
			} else {
				ret -= p[i + 1];
			}
		}
		Some(ret)
	}

	fn is_digit(ch: u8) -> bool {
		b'0' <= ch && ch <= b'9'
	}
}