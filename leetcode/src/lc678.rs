use std::hint::unreachable_unchecked;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
	    let s = s.as_bytes();
	    let mut arr = Vec::with_capacity(s.len());
	    for i in 0..s.len() {
		    match s[i] {
			    b'(' => arr.push(b'('),
			    b'*' => arr.push(b'*'),
			    b')' => {
				    let mut k = arr.iter().rposition(|&e| e == b'(');
				    if let Some(k) = k {
					    arr.remove(k);
				    } else if arr.len() != 0 {
					    arr.pop();
				    } else {
					    return false;
				    }
			    }
			    _ => unreachable!()
		    }
	    }
	    let mut cnt = 0;
	    for i in (0..arr.len()).rev() {
		    match arr[i] {
			    b'(' => {
				    if cnt == 0 {
					    return false;
				    }
				    cnt -= 1;
			    }
			    b'*' => {
				    cnt += 1;
			    }
			    _ => unreachable!(),
		    }
	    }
	    true
    }
}
