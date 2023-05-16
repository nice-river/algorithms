#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let mut ans = 0;
        Self::dfs(0, &cost, &mut ans);
        ans
    }

    fn dfs(root: usize, cost: &Vec<i32>, ans: &mut i32) -> i32 {
        if root >= cost.len() {
            return 0;
        }
        let a = Self::dfs((root + 1) * 2 - 1, cost, ans);
        let b = Self::dfs((root + 1) * 2, cost, ans);
        *ans += (a - b).abs();
        cost[root] + a.max(b)
    }
}
