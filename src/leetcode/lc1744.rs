struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut pre_sum = vec![0; candies_count.len() + 1];
		for i in 0..candies_count.len() {
			pre_sum[i+1] = pre_sum[i] + candies_count[i] as i64;
		}
		let mut ans = Vec::with_capacity(queries.len());

		for query in queries {
            let fav_ty = query[0] as usize;
			let fav_day = query[1] as i64;
			let day_cap = query[2] as i64;
            if day_cap * fav_day + day_cap - 1 >= pre_sum[fav_ty] && pre_sum[fav_ty + 1] >= fav_day + 1 {
                ans.push(true);
			} else {
				ans.push(false);
			}
		}

		ans
    }
}