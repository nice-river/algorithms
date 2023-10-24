#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aababb".to_owned();
        let ans = 2;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test1() {
        let s = "abcde".to_owned();
        let ans = 0;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test2() {
        let s = "isaxakdkzcwkjmpzcnum".to_owned();
        let ans = 2;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test3() {
        let s = "ykudzhiixwttnvtesiwnbcjmsydidttiyabbwzlfbmmycwjgzwhbtvtxyvkkjgfehaypiygpstkhakfasiloaveqzcywsiujvixcdnxpvvtobxgroznswwwipypwmdhldsoswrzyqthaqlbwragjrqwjxgmftjxqugoonxadazeoxalmccfeyqtmoxwbnphxih".to_owned();
        let ans = 12;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test4() {
        let s = "aabaabaa".to_owned();
        let ans = 4;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test5() {
        let s = "ababb".to_owned();
        let ans = 2;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test6() {
        let s = "aab".to_owned();
        let ans = 1;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

use crate::*;

struct Solution {}

const ALPHA: usize = 26;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut cnts = vec![0; ALPHA];
        let mut stks: Vec<Vec<Vec<(i32, usize)>>> = vec![vec![vec![]; ALPHA]; ALPHA];
        for i in 0..ALPHA {
            for j in 0..ALPHA {
                stks[i][j].push((0, 0));
            }
        }
        let mut pos = vec![0; ALPHA];
        for i in 0..s.len() {
            let x = (s[i] - b'a') as usize;
            cnts[x] += 1;
            pos[x] = i + 1;
            for y in 0..ALPHA {
                if y == x {
                    continue;
                }
                let stk = &stks[x][y];
                let mut l = 0;
                let mut r = stk.len();
                let p = pos[y];
                let mut q = stk.len();
                while l < r {
                    let m = (l + r) / 2;
                    if stk[m].1 < p {
                        q = m;
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                if q != stk.len() {
                    ans = ans.max(cnts[x] - cnts[y] - stk[q].0);
                }
                let stk = &stks[y][x];
                let mut l = 0;
                let mut r = stk.len();
                let p = pos[y];
                let mut q = stk.len();
                while l < r {
                    let m = (l + r) / 2;
                    if stk[m].1 < p {
                        q = m;
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                if q != stk.len() {
                    ans = ans.max(cnts[y] - cnts[x] - stk[q].0);
                }
                if stks[x][y].last().unwrap().0 > cnts[x] - cnts[y] {
                    stks[x][y].push((cnts[x] - cnts[y], i + 1));
                }
                if stks[y][x].last().unwrap().0 > cnts[y] - cnts[x] {
                    stks[y][x].push((cnts[y] - cnts[x], i + 1));
                }
            }
        }
        ans
    }
}
