use std::collections::BinaryHeap;

struct KthLargest {
	cap: usize,
	heap: BinaryHeap<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

	fn new(k: i32, nums: Vec<i32>) -> Self {
		let cap = k as usize;
		let mut heap = BinaryHeap::new();
		for num in nums {
			heap.push(-num);
			if heap.len() > cap {
				heap.pop();
			}
		}
		Self {
			cap,
			heap
		}
	}

	fn add(&mut self, val: i32) -> i32 {
		self.heap.push(-val);
		if self.heap.len() > self.cap {
			self.heap.pop();
		}
		-*self.heap.peek().unwrap()
	}
}
