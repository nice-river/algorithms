#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let routes = vec![vec![1,2,7], vec![3,6,7]];
		let source = 1;
		let target = 6;
		let ans = 2;
		assert_eq!(Solution::num_buses_to_destination(routes, source, target), ans);
	}

	#[test]
	fn test1() {
		let routes = vec![vec![7,12],vec![4,5,15],vec![6],vec![15,19],vec![9,12,13]];
		let source = 15;
		let target = 12;
		let ans = -1;
		assert_eq!(Solution::num_buses_to_destination(routes, source, target), ans);
	}

	#[test]
	fn test2() {
		let routes = vec![vec![7,12],vec![4,5,15],vec![6],vec![15,19],vec![9,12,13]];
		let source = 7;
		let target = 12;
		let ans = 1;
		assert_eq!(Solution::num_buses_to_destination(routes, source, target), ans);
	}
}

struct Solution {}

use std::collections::HashMap;
use std::collections::VecDeque;

struct Node {
	bits: HashMap<u64, u64>,
}

impl Node {
	fn new(route: &Vec<i32>) -> Self {
		let mut bits = HashMap::new();
		for &r in route {
			let r = r as u64;
			let p = r / 64;
			let q = r % 64;
			*bits.entry(p).or_insert(0) |= (1 << q);
		}
		Self {
			bits
		}
	}

	fn intersect(&self, oth_node: &Self) -> bool {
		let (a, b) = if self.bits.len() < oth_node.bits.len() {
			(&self.bits, &oth_node.bits)
		} else {
			(&oth_node.bits, &self.bits)
		};
		for (p, q) in a.iter() {
			if let Some(x) = b.get(p) {
				if *q & *x != 0 {
					return true;
				}
			}
		}
		false
	}
}


impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
	    if source == target {
		    return 0;
	    }
	    let n = routes.len();
	    let mut nodes = Vec::with_capacity(n);
	    for route in routes.iter() {
		    nodes.push(Node::new(route));
	    }
	    let mut graph = vec![vec![]; n];
	    for i in 0..n {
		    for j in i+1..n {
			    if nodes[i].intersect(&nodes[j]) {
				    graph[i].push(j);
				    graph[j].push(i);
			    }
		    }
	    }
	    let mut target_routes = HashMap::new();
	    let mut vis = vec![false; n];
	    let mut que = VecDeque::new();
	    for (i, route) in routes.iter().enumerate() {
		    for &r in route {
			    if r == source {
				    que.push_back((i, 1));
				    vis[i] = true;
			    }
			    if r == target {
				    target_routes.insert(i, ());
			    }
		    }
	    }
	    while !que.is_empty() {
		    let (route, step) = que.pop_front().unwrap();
		    if target_routes.contains_key(&route) {
			    return step;
		    }
		    for &nxt_route in graph[route].iter() {
			    if !vis[nxt_route] {
				    vis[nxt_route] = true;
				    que.push_back((nxt_route, step + 1));
			    }
		    }
	    }
	    -1
    }
}