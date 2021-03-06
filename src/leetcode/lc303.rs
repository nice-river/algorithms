struct NumArray {
	arr: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

	fn new(nums: Vec<i32>) -> Self {
		let mut arr = vec![0; nums.len() + 1];
		for i in 0..nums.len() {
			arr[i + 1] = arr[i] + nums[i];
		}
		Self {
			arr
		}
	}

	fn sum_range(&self, i: i32, j: i32) -> i32 {
		self.arr[j as usize + 1]  - self.arr[i as usize]
	}
}
