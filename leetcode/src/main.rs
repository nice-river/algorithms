mod offer20;

fn main() {
	let s = "-1E-16 ".into();
	println!("{}", offer20::Solution::is_number(s));
}