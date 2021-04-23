struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let haystack = "mississippi".to_string();
        let needle = "issip".to_string();
        let expected = 4;
        assert_eq!(Solution::str_str(haystack, needle), expected);
	}
}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let mut next = vec![0; needle.len()];
        for i in 1..next.len() {
            let mut j = next[i - 1];
            while j != 0 {
                if needle[j] == needle[i] {
                    break;
                }
                j = next[j - 1];
            }
            next[i] = j + if needle[j] == needle[i] { 1 } else { 0 };
        }
        let (mut i, mut j) = (0, 0);
        while i < haystack.len() && j < needle.len() {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;
            } else if j == 0 {
                i += 1;
            } else {
                j = next[j - 1];
            }
        }
        if j == needle.len() {
            (i - needle.len()) as i32
        } else {
            -1
        }
    }
}