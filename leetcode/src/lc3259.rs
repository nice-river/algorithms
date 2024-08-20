#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
	let n = energy_drink_a.len();
	let mut dp0 = vec![0; n];
	let mut dp1 = vec![0; n];
	dp0[0] = energy_drink_a[0] as i64;
	dp1[0] = energy_drink_b[0] as i64;
	for i in 1..n {
	    dp0[i] = (dp0[i - 1] + energy_drink_a[i] as i64).max(dp1[i - 1]);
	    dp1[i] = (dp1[i - 1] + energy_drink_b[i] as i64).max(dp0[i - 1]);
	}
	dp0[n - 1].max(dp1[n - 1])
    }
}
