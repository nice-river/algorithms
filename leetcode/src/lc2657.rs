#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut ans = vec![];
        for i in 0..n {
            let mut t = 0;
            for x in &a[0..=i] {
                for y in &b[0..=i] {
                    if x == y {
                        t += 1;
                    }
                }
            }
            ans.push(t);
        }
        ans
    }
}
