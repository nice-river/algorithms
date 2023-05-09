#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let tests = vec![(1, 1), (8, 8), (11, 0), (13, 1)];
        for (n, ans) in tests.into_iter() {
            assert_eq!(Solution::find_nth_digit(n), ans);
        }
    }

    #[test]
    fn test1() {
        let tests = vec![(490000, 1), (3487934, 4), (488889, 9)];
        for (n, ans) in tests.into_iter() {
            assert_eq!(Solution::find_nth_digit(n), ans);
        }
    }
}

struct Solution {}

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        Solution::dfs(1, n as i64)
    }

    fn dfs(digit_cnt: u32, n: i64) -> i32 {
        let tot_num = 10i64.pow(digit_cnt) - 10i64.pow(digit_cnt - 1);
        let tot_cnt = tot_num * digit_cnt as i64;
        dbg!(digit_cnt, n, tot_cnt, tot_num);
        if n > tot_cnt {
            Solution::dfs(digit_cnt + 1, n - tot_cnt)
        } else {
            let digit_idx = (n - 1) % digit_cnt as i64;
            let num = 10i64.pow(digit_cnt - 1) + (n - 1) / digit_cnt as i64;
            dbg!(num, digit_idx);
            (num.to_string().as_bytes()[digit_idx as usize] - b'0') as i32
        }
    }
}
