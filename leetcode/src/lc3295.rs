#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
	let set = HashSet::<String>::from_iter(banned_words.into_iter());
	let mut cnt = 0;
	for message in message { 
	    if set.contains(&message) {
		cnt += 1;
	    }
	    if cnt >= 2 {
		return true;
	    }
	}
	false
    }
}
