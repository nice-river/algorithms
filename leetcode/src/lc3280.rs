#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
	let mut ans = vec![];
	for x in date.split('-') {
	    let x = x.parse::<i32>().unwrap();
	    ans.push(format!("{x:b}"));
	}
	ans.join("-")
    }
}
