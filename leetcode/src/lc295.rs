struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


use std::collections::BinaryHeap;

struct MedianFinder {
	smaller: BinaryHeap<i32>,
	larger: BinaryHeap<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
			Self {
				smaller: BinaryHeap::new(),
				larger: BinaryHeap::new(),
			}
    }
    
    fn add_num(&mut self, num: i32) {
			if self.smaller.len() > self.larger.len() {
				self.smaller.push(num);
				let x = self.smaller.pop().unwrap();
				self.larger.push(-x);
			} else {
				if self.smaller.len() == 0 {
					self.smaller.push(num);
				} else {
					if num < *self.smaller.peek().unwrap() {
						self.smaller.push(num);
					} else {
						self.larger.push(-num);
						self.smaller.push(-self.larger.pop().unwrap());
					}
				}
			}
    }
    
    fn find_median(&self) -> f64 {
			if self.smaller.len() > self.larger.len() {
				let x = self.smaller.peek().unwrap();
				*x as f64
			} else {
				let x = *self.smaller.peek().unwrap();
				let y = -*self.larger.peek().unwrap();
				(x as f64 + y as f64) / 2.0
			}
    }
}