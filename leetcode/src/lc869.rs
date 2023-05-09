#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 46;
        assert!(Solution::reordered_power_of2(46));
    }
}

struct Solution {}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut p = n.to_string().into_bytes();
        p.sort();
        for i in 0..=30 {
            let mut q = (1 << i).to_string().into_bytes();
            q.sort();
            if p == q {
                return true;
            }
        }
        false
    }
}
