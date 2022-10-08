struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let equations = vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()]];
		let values = vec![2.0, 3.0];
		let queries = vec![vec!["x".to_string(), "x".to_string()]];
		assert_eq!(Solution::calc_equation(equations, values, queries), vec![-1.0]);
	}
}

use std::collections::{HashMap, HashSet};

impl Solution {
	pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
		let mut map = HashMap::new();
		for i in 0..equations.len() {
			let a = equations[i][0].clone();
			let b = equations[i][1].clone();
			let v = values[i];
			let mut entry = map.entry(a.clone()).or_insert_with(|| vec![]);
			entry.push((b.clone(), v));
			let mut entry = map.entry(b).or_insert_with(|| vec![]);
			entry.push((a, 1.0 / v));
		}
		let mut ans = vec![];
		for query in queries {
			let mut vis = HashSet::new();
			vis.insert(query[0].as_str());
			let ret = Solution::find_val(&query[0], &query[1], &map, &mut vis, 1.0).unwrap_or(-1.0);
			ans.push(ret);
		}
		ans
	}

	fn find_val<'a, 'b: 'a>(cur: &'a str, target: &str, map: &'b HashMap<String, Vec<(String, f64)>>, vis: &mut HashSet<&'a str>, val: f64) -> Option<f64> {
		if cur == target {
			return if map.contains_key(cur) {Some(val)} else {None};
		}
		let children = map.get(cur)?;
		for ch in children.iter() {
			if !vis.contains(ch.0.as_str()) {
				vis.insert(cur);
				let ret = Solution::find_val(&ch.0, target, map, vis, val * ch.1);
				vis.remove(cur);
				if ret.is_some() {
					return ret;
				}
			}
		}
		None
	}
}