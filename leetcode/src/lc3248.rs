#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
	let (mut a, mut b) = (0, 0);
	for command in commands {
	    match command.as_str() {
		"RIGHT" => b += 1,
		"LEFT" => b -= 1,
		"DOWN" => a += 1,
		"UP" => a -= 1,
		_ => unreachable!()
	    }
	}

	a * n + b
    }
}
