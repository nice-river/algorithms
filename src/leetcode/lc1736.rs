#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn maximum_time(time: String) -> String {
	    let mut time = time.into_bytes();
	    if time[0] == b'?' {
		    time[0] = if time[1] != b'?' && time[1] >= b'4' {
			    b'1'
		    } else {
			    b'2'
		    }
	    }
	    if time[1] == b'?' {
		    time[1] = if time[0] <= b'1' {
			    b'9'
		    } else {
			    b'3'
		    }
	    }
	    if time[3] == b'?' {
		    time[3] = b'5';
	    }
	    if time[4] == b'?' {
		    time[4] = b'9';
	    }
	    String::from_utf8(time).unwrap()
    }
}