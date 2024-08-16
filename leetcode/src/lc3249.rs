#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
	let edges = to_vec!( [[0,1],[1,2],[2,3],[3,4],[0,5],[1,6],[2,7],[3,8]]);
	let ans = 6;
	assert_eq!(Solution::count_good_nodes(edges), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
	let mut tree = vec![vec![]; edges.len() + 1];
	for edge in edges {
	    let u = edge[0] as usize;
	    let v = edge[1] as usize;
	    tree[u].push(v);
	    tree[v].push(u);
	}
	let mut ans = 0;
	Self::dfs(0, tree.len(), &tree, &mut ans);
	ans
    }

    fn dfs(root: usize, parent: usize, tree: &Vec<Vec<usize>>, ans: &mut i32) -> i32 {
	let mut ret = 0;
	let mut ns = vec![];
	for &ch in &tree[root] {
	    if ch == parent {
		continue;
	    }
	    let num = Self::dfs(ch, root, tree, ans);
	    ns.push(num);
	    ret += num;
	}
	*ans += 1;
	for i in 1..ns.len() {
	    if ns[i] != ns[i - 1] {
		*ans -= 1;
		break;
	    }
	}
	ret + 1
    }
}
