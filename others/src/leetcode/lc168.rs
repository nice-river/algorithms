struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let column_number = 28;
		let ans = String::from("AB");
		assert_eq!(Solution::convert_to_title(column_number), ans);
	}
}

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
		column_number -= 1;
		let mut ans = Vec::new();
		loop {
            let a = column_number / 26;
			let b = column_number % 26;
			ans.push(String::from((b as u8 + b'A') as char));
            if a == 0 {
				break;
			}
			column_number = a - 1;
		}
        ans.reverse();
        ans.join("")
    }
}