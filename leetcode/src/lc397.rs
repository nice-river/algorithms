#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 15;
        let ans = 5;
        assert_eq!(Solution::integer_replacement(n), ans);
    }
}

struct Solution {}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut que = VecDeque::new();
        let mut set = HashSet::new();
        que.push_back((n as i64, 0));
        while !que.is_empty() {
            let (n, s) = que.pop_front().unwrap();
            if n == 1 {
                return s;
            }
            if n % 2 == 0 {
                let n = n / 2;
                if !set.contains(&n) {
                    que.push_back((n, s + 1));
                    set.insert(n);
                }
            } else {
                let a = n + 1;
                if !set.contains(&a) {
                    que.push_back((a, s + 1));
                    set.insert(a);
                }
                let b = n - 1;
                if !set.contains(&b) {
                    que.push_back((b, s + 1));
                    set.insert(b);
                }
            }
        }
        -1
    }
}
