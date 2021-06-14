#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let cost = vec![4,3,2,5,6,7,2,5,5];
		let target = 9;
		let ans = String::from("7772");
		assert_eq!(Solution::largest_number(cost, target), ans);
	}

	#[test]
	fn test1() {
		let cost = vec![2,4,6,2,4,6,4,4,4];
		let target = 5;
		let ans = String::from("0");
		assert_eq!(Solution::largest_number(cost, target), ans);
	}
}

struct Solution {}

use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
	    let mut map = HashMap::new();
	    for i in 0..cost.len() {
		    map.insert(cost[i] as usize, i + 1);
	    }
	    let target = target as usize;
	    let mut dp = vec![None; target + 1];
	    dp[0] = Some(Node::default());
	    for (k, v) in map.into_iter() {
		    if k > target {
			    continue;
		    }
		    for i in 0..=target-k {
			    if let Some(node) = dp[i].as_ref() {
				    let mut x = node.clone();
				    x.sz += 1;
				    x.digits[v] += 1;
				    if dp[i + k].is_none() {
					    dp[i + k] = Some(x);
				    } else {
					    dp[i + k] = Some(dp[i + k].unwrap().max(x));
				    }
			    }
		    }
	    }
	    if let Some(node) = dp[target] {
		    let mut ans = Vec::with_capacity(node.sz);
		    for i in (1..=9).rev() {
			    for _ in 0..node.digits[i] {
				    ans.push(i.to_string());
			    }
		    }
		    ans.join("")
	    } else {
		    String::from("0")
	    }
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq)]
struct Node {
	sz: usize,
	digits: [i32; 10],
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.sz.cmp(&other.sz) {
			Ordering::Less => Some(Ordering::Less),
			Ordering::Equal => {
				for i in (1..=9).rev() {
					if self.digits[i] < other.digits[i] {
						return Some(Ordering::Less);
					} else if self.digits[i] > other.digits[i] {
						return Some(Ordering::Greater);
					}
				}
				Some(Ordering::Equal)
			}
			Ordering::Greater => Some(Ordering::Greater),
		}
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}