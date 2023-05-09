#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}


impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
	    let mut ans = vec![];
	    for hour in 0..12i32 {
		    for minute in 0..60i32 {
			    if hour.count_ones() + minute.count_ones() == turned_on as u32 {
				    ans.push(format!("{}:{:02}", hour, minute));
			    }
		    }
	    }
	    ans
    }
}