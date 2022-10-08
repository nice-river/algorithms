#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut s = RandomizedCollection::new();
		s.insert(4);
		s.insert(3);
		s.insert(4);
		s.insert(2);
		s.insert(4);
		s.remove(4);
		s.remove(3);
		s.remove(4);
		s.remove(4);
	}
}


use std::collections::HashMap;
use std::collections::HashSet;
use rand;

struct RandomizedCollection {
	mm: HashMap<i32, HashSet<usize>>,
	arr: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {

	/** Initialize your data structure here. */
	fn new() -> Self {
		RandomizedCollection {
			mm: HashMap::new(),
			arr: vec![],
		}
	}

	/** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
	fn insert(&mut self, val: i32) -> bool {
		let mut e = self.mm.entry(val).or_insert(HashSet::new());
		let ret = e.is_empty();
		self.arr.push(val);
		e.insert(self.arr.len() - 1);
		ret
	}

	/** Removes a value from the collection. Returns true if the collection contained the specified element. */
	fn remove(&mut self, val: i32) -> bool {
		if !self.mm.contains_key(&val) {
			return false;
		}
		let mut e = self.mm.entry(val).or_insert(HashSet::new());
		if e.is_empty() {
			return false;
		}
		let p = e.iter().next().cloned().unwrap();
		e.remove(&p);
		let s = self.arr.last().unwrap();
		if p != self.arr.len() - 1 {
			self.mm.get_mut(s).unwrap().remove(&(self.arr.len() - 1));
			self.mm.get_mut(s).unwrap().insert(p);
		}
		self.arr.swap_remove(p);
		true
	}

	/** Get a random element from the collection. */
	fn get_random(&self) -> i32 {
		if self.arr.is_empty() {
			return 0;
		}
		let idx = rand::random::<usize>() % self.arr.len();
		self.arr[idx]
	}
}
