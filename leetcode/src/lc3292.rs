#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}


use crate::*;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
	todo!()
    }
}

use std::ops::{Add, Mul};

struct Solution {}

struct StrHashVal {
    vals: Vec<i64>,
}

impl StrHashVal {
    const ALPHABET: i64 = 26;
    const MODULES: [i64; 1] = [1125899906842597];

    fn from_str<S: AsRef<str>>(s: S) -> Self {
	let mut vals = vec![];
	for module in Self::MODULES {
	    let mut v = 0;
	    for &c in s.as_ref().as_bytes() {
		v = (v * Self::ALPHABET) % module + c as i64;
	    }
	    vals.push(v);
	}
	Self { vals }
    }
}


impl<T> Add<T> for StrHashVal where T: Into<i64> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
	let rhs: i64 = rhs.into();
	Self {

	}
	for (v, module) in self.vals.iter_mut().zip(Self::MODULES.iter()) {
	    *v = (*v + rhs) % *module;
	}
	self
    }
}


impl<T> Mul<T> for StrHashVal where T: Into<i64> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
	let rhs: i64 = rhs.into();
	for (v, module) in self.vals.iter_mut().zip(Self::MODULES.iter()) {
	    *v = (*v + rhs) % *module;
	}
	self
    }
}


