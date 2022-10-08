#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        let mut c = 1;
        let mut i = 0;
        while c != 0 && i < digits.len() {
            c = (digits[i] + 1) / 10;
            digits[i] = (digits[i] + 1) % 10;
            i += 1;
        }
        if c != 0 {
            digits.push(1);
        }
        digits.reverse();
        digits
    }
}
