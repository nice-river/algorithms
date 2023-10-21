#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

impl Solution {
    pub fn max_product(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut a = vec![];
        let mut b = vec![];
        Solution::dfs(0, s, &mut a, &mut b, &mut ans);
        ans as i32
    }

    fn dfs(p: usize, s: &[u8], a: &mut Vec<u8>, b: &mut Vec<u8>, ans: &mut usize) {
        if p == s.len() {
            if Self::is_palidrome(a) && Self::is_palidrome(b) {
                *ans = (*ans).max(a.len() * b.len());
            }
            return;
        }
        a.push(s[p]);
        Solution::dfs(p + 1, s, a, b, ans);
        a.pop();
        b.push(s[p]);
        Solution::dfs(p + 1, s, a, b, ans);
        b.pop();
        Solution::dfs(p + 1, s, a, b, ans);
    }

    fn is_palidrome(a: &Vec<u8>) -> bool {
        for i in 0..a.len() / 2 {
            if a[i] != a[a.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}
