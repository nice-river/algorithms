#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 2;
        let ans = "11".to_string();
        assert_eq!(Solution::count_and_say(n), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = "1".to_string();
        for _ in 1..n {
            let b = s.as_bytes();
            let mut i = 0;
            let mut t = String::new();
            for j in 1..=b.len() {
                if j == b.len() || b[j] != b[j - 1] {
                    t.push_str(&(j - i).to_string());
                    t.push(b[i] as char);
                    i = j;
                }
            }
            s = t;
        }
        s
    }
}
