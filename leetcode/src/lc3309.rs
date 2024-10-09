#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
	let nums = vec![1, 2, 3];
	let ans = 30;
	assert_eq!(Solution::max_good_number(nums), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_good_number(mut nums: Vec<i32>) -> i32 {
	nums.sort_by(|a, b| {
	    let p = Self::foo(&[*a, *b]);
	    let q = Self::foo(&[*b, *a]);
	    q.cmp(&p)
	});
	Self::foo(&nums)
    }

    fn foo(arr: &[i32]) -> i32 {
	let s = arr.iter().map(|v| format!("{:b}", v)).collect::<Vec<_>>().join("");
        i32::from_str_radix(&s, 2).unwrap()
    }
}
