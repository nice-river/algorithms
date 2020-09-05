mod offer20;
mod leetcode;

fn main() {
	let s = "-1E-16 ".into();
	println!("{}", offer20::Solution::is_number(s));
}