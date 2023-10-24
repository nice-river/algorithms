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
        let s = "abbbbabbbb".to_owned();
        let ans = 7;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test6() {
        let s = "aab".to_owned();
        let ans = 1;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test7() {
        let s = "aaaa".to_owned();
        let ans = 0;
        assert_eq!(Solution::largest_variance(s), ans);
    }

    #[test]
    fn test8() {
        let s = "aabbbbaa".to_owned();
        let ans = 3;
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
        let mut pos = vec![vec![]; ALPHA];
        for (i, &ch) in s.iter().enumerate() {
            pos[(ch - b'a') as usize].push(i);
        }

        for i in 0..ALPHA {
            for j in 0..ALPHA {
                if i == j {
                    continue;
                }
                let mut p = 0;
                let mut q = 0;
                let mut f = -1;
                let mut g = i32::MIN;
                while p < pos[i].len() && q < pos[j].len() {
                    if pos[i][p] < pos[j][q] {
                        f = f.max(0) + 1;
                        g += 1;
                        p += 1;
                    } else {
                        g = g.max(f).max(0) - 1;
                        f = f.max(0) - 1;
                        q += 1;
                    }
                    ans = ans.max(g);
                }
                while p < pos[i].len() {
                    f = f.max(0) + 1;
                    g += 1;
                    p += 1;
                    ans = ans.max(g);
                }
                while q < pos[j].len() {
                    g = g.max(f).max(0) - 1;
                    f = f.max(0) - 1;
                    q += 1;
                    ans = ans.max(g);
                }
            }
        }
        ans
    }
}
