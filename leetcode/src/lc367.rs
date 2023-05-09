#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut l = 0;
        let mut r = num.max(2);
        while l + 1 < r {
            let m = l + (r - l) / 2;
            if let Some(x) = m.checked_mul(m) {
                if x > num {
                    r = m;
                } else {
                    l = m;
                }
            } else {
                r = m;
            }
        }
        l * l == num
    }
}
