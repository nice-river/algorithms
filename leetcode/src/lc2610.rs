#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut c = 0;
        for num in nums {
            let g = map.entry(num).or_insert(0);
            *g += 1;
            c = c.max(*g);
        }
        let mut ans = vec![vec![]; c as usize];
        for (&k, &v) in map.iter() {
            for i in 0..v as usize {
                ans[i].push(k);
            }
        }
        ans
    }
}
