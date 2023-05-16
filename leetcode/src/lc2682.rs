#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        ans[0] = 1;
        let mut p = 0;
        for i in 1.. {
            let t = (p + i * k) % n;
            if ans[t as usize] == 1 {
                break;
            }
            ans[t as usize] = 1;
            p = t;
        }

        ans.into_iter()
            .enumerate()
            .filter_map(|(i, v)| if v == 0 { Some(i as i32 + 1) } else { None })
            .collect::<Vec<i32>>()
    }
}
