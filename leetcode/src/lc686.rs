#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = "abcd";
        let b = "cdabcdab";
        let ans = 3;
        assert_eq!(Solution::repeated_string_match(a.into(), b.into()), ans);
    }

    #[test]
    fn test1() {
        let a = "abcdefg";
        let b = "fgabcdefga";
        let ans = 3;
        assert_eq!(Solution::repeated_string_match(a.into(), b.into()), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let k = (b.len() - 1) / a.len() + 2;
        let r = a.repeat(k);
        if let Some(p) = r.find(&b) {
            let k = b.len().checked_sub(a.len() - p).unwrap_or(0);
            1 + (k / a.len()) as i32 + if k % a.len() == 0 { 0 } else { 1 }
        } else {
            -1
        }
    }
}
