struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
		for word in words {
			*map.entry(word).or_insert(0) += 1;
		}
		let mut heap = BinaryHeap::new();
		for (s, cnt) in map.into_iter() {
			heap.push((Reverse(cnt), s));
			if heap.len() > k as usize {
				heap.pop();
			}
		}
		let mut ans = Vec::with_capacity(heap.len());
        while let Some((_, s)) = heap.pop() {
			ans.push(s);
		}
		ans.reverse();
		ans
    }
}