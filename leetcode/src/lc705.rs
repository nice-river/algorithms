struct MyHashSet {
	elem_arr: Vec<bool>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
	/** Initialize your data structure here. */
	fn new() -> Self {
		Self {
			elem_arr: vec![false; 1000001],
		}
	}

	fn add(&mut self, key: i32) {
		self.elem_arr[key as usize] = true;
	}

	fn remove(&mut self, key: i32) {
		self.elem_arr[key as usize] = false;
	}

	fn contains(&self, key: i32) -> bool {
		self.elem_arr[key as usize]
	}
}