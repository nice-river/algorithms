#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut reward = reward1
            .iter()
            .zip(reward2.iter())
            .enumerate()
            .map(|(i, d)| (d.1 - d.0, i))
            .collect::<Vec<_>>();
        reward.sort_unstable();
        let mut ans = 0;
        for i in 0..k as usize {
            ans += reward1[reward[i].1];
        }
        for i in k as usize..reward.len() {
            ans += reward2[reward[i].1];
        }
        ans
    }
}
