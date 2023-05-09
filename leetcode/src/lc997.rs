#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut in_cnt = vec![0; n as usize + 1];
        let mut out_cnt = vec![0; n as usize + 1];
        for p in trust {
            in_cnt[p[1] as usize] += 1;
            out_cnt[p[0] as usize] += 1;
        }
        let mut ans = -1;
        for i in 1..=n as usize {
            if in_cnt[i] == n - 1 && out_cnt[i] == 0 {
                ans = i as i32;
                break;
            }
        }
        ans
    }
}
