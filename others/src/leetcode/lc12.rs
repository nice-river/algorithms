struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let num = 9;
		let ans = "IX";
		assert_eq!(Solution::int_to_roman(num), ans.to_string());
	}
}


impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
		let symbols = [("I", "V", "X"), ("X", "L", "C"), ("C", "D", "M"), ("M", "", "")];
		let mut rs = vec![];
		let mut i = 0;
        while num != 0 {
			let digit = num % 10;
            rs.push(Solution::to_roman(digit as usize, symbols[i].0, symbols[i].1, symbols[i].2));
            num /= 10;
            i += 1;
		}
		rs.reverse();
		rs.join("")
    }

	fn to_roman(digit: usize, r1: &str, r5: &str, r10: &str) -> String{
		return if digit < 4 {
			r1.repeat(digit)
		} else if digit == 4 {
			format!("{}{}", r1, r5)
		} else if digit < 9 {
			format!("{}{}", r5, r1.repeat(digit - 5))
		} else if digit == 9 {
			format!("{}{}", r1, r10)
		} else {
			r10.to_string()
		}
	}
}