#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut min_a = m;
        let mut min_b = n;
        for op in ops {
            min_a = min_a.min(op[0]);
            min_b = min_b.min(op[1]);
        }
        min_a * min_b
    }
}
