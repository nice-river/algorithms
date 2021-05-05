#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
	    let mut s = x.to_string();
	    let x = unsafe {s.as_bytes_mut()};
        match x[0] {
	        b'-' => (&mut x[1..]).reverse(),
	        _ => x.reverse(),
        }
        s.parse::<i32>().unwrap_or(0)
    }
}