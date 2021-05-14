struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xors = vec![0; arr.len() + 1];
		for i in 0..arr.len() {
			xors[i + 1] = xors[i] ^ arr[i];
		}
		queries.into_iter().map(|query| xors[query[1] as usize + 1] ^ xors[query[0] as usize]).collect()
    }
}