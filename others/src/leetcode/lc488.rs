#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let board = "G";
        let hand = "GGGGG";
        let ans = 2;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test1() {
        let board = "RBYYBBRRB";
        let hand = "YRBGB";
        let ans = 3;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test2() {
        let board = "WRRBBW";
        let hand = "RB";
        let ans = -1;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test3() {
        let board = "WRRBBW";
        let hand = "RBW";
        let ans = 3;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test4() {
        let board = "WWRRBBWW";
        let hand = "WRBRW";
        let ans = 2;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test5() {
        let board = "RRBB";
        let hand = "RB";
        let ans = 2;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test6() {
        let board = "RRWWRRBBRR";
        let hand = "BW";
        let ans = 2;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test7() {
        let boards = ["GGRRWWRRWWRRG", "RRGGBBYYWWRRGGBB"];
        let hands = ["WW", "RGBYW"];
        let ans = [2, -1];
        for i in 0..ans.len() {
            assert_eq!(
                Solution::find_min_step(boards[i].to_string(), hands[i].to_string()),
                ans[i]
            );
        }
    }

    #[test]
    fn test8() {
        let board = "RRYGGYYRRYYGGYRR";
        let hand = "GGBBB";
        let ans = 5;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test9() {
        let board = "BB";
        let hand = "BB";
        let ans = 1;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }

    #[test]
    fn test10() {
        let board = "WWRRWW";
        let hand = "R";
        let ans = 1;
        assert_eq!(
            Solution::find_min_step(board.to_string(), hand.to_string()),
            ans
        );
    }
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let board = board.into_bytes();
        let hand = hand.into_bytes();
        let mut dp = HashMap::new();
        let mut ans = Solution::dfs(board, &hand, 0, &mut dp);
        ans as i32
    }

    fn dfs(board: Vec<u8>, hand: &Vec<u8>, used: i32, dp: &mut HashMap<Vec<u8>, u32>) -> u32 {
        if board.len() == 0 {
            return 0;
        }
        if let Some(&v) = dp.get(&board) {
            return v;
        }
        let n = board.len();
        let mut ans = u32::MAX;
        for i in 0..hand.len() {
            if (1 << i) & used != 0 {
                continue;
            }
            for j in 0..n {
                let mut ok = false;
                if j > 0 && board[j] == board[j - 1] && board[j - 1] != hand[i] {
                    ok = true;
                }
                if board[j] == hand[i] {
                    ok = true;
                }
                if !ok {
                    continue;
                }
                let mut seq = (&board[0..j]).to_vec();
                seq.push(hand[i]);
                if j != n {
                    seq.extend_from_slice(&board[j..])
                }
                let mut k = j;
                while 0 <= k && k < seq.len() {
                    let c = seq[k];
                    let mut l = k;
                    let mut r = k;
                    while l > 0 && seq[l - 1] == c {
                        l -= 1;
                    }
                    while r < seq.len() && seq[r] == c {
                        r += 1;
                    }
                    if r - l >= 3 {
                        seq.drain(l..r);
                        k = if l > 0 { l - 1 } else { seq.len() };
                    } else {
                        break;
                    }
                }
                ans = ans.min(
                    Solution::dfs(seq, &hand, used | (1 << i), dp)
                        .checked_add(1)
                        .unwrap_or(u32::MAX),
                );
            }
        }
        dp.insert(board, ans);
        ans
    }
}
