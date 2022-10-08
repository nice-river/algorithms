struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let customers = vec![1,0,1,2,1,1,7,5];
		let grumpy = vec![0,1,0,1,0,1,0,1];
		let x = 3;
		assert_eq!(Solution::max_satisfied(customers, grumpy, x), 16);
	}
}

impl Solution {
	pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
		let n = customers.len();
		let mut pre_sum = vec![0; n + 1];
		let mut tot_pre_sum = vec![0; n + 1];
		for i in 0..n {
			pre_sum[i + 1] = pre_sum[i] + if grumpy[i] == 1 { 0 } else { customers[i] };
			tot_pre_sum[i + 1] = tot_pre_sum[i] + customers[i];
		}
		let mut ans = 0;
		let x = x as usize;
		for i in 0..n - x + 1{
			let g = tot_pre_sum[i + x] - tot_pre_sum[i];
			let t = pre_sum[i + x] - pre_sum[i];
			ans = ans.max(pre_sum[n] + g - t);
		}
		ans
	}
}