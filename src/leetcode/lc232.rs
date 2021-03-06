use std::collections::VecDeque;

struct MyQueue {
	que: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

	/** Initialize your data structure here. */
	fn new() -> Self {
		Self {
			que: VecDeque::new(),
		}
	}

	/** Push element x to the back of queue. */
	fn push(&mut self, x: i32) {
		self.que.push_back(x);
	}

	/** Removes the element from in front of queue and returns that element. */
	fn pop(&mut self) -> i32 {
		self.que.pop_front().unwrap()
	}

	/** Get the front element. */
	fn peek(&self) -> i32 {
		*self.que.front().unwrap()
	}

	/** Returns whether the queue is empty. */
	fn empty(&self) -> bool {
		self.que.is_empty()
	}
}