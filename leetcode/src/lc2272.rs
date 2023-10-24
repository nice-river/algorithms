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

use std::collections::BTreeSet;

const ALPHA: usize = 26;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut sets: Vec<BTreeSet<i32>> = vec![BTreeSet::new(); ALPHA];
        for i in 0..ALPHA {
            sets[i].insert(0);
        }
        let mut cnts = vec![0i32; ALPHA];
        for &x in s.iter() {
            let x = (x - b'a') as usize;
            cnts[x] += 1;
            let mut it = sets[x].iter();
            let mut succ = false;
            let mut p = 0;
            while let Some(&v) = it.next() {
                if v == cnts[x] {
                    continue;
                } else {
                    succ = true;
                    p += v;
                    break;
                }
            }
            if succ {
                for y in b'a'..=b'z' {
                    let y = (y - b'a') as usize;
                    if x == y {
                        continue;
                    }
                    let d = cnts[x] - cnts[y];
                    let mut it = sets[y].iter().rev();
                    succ = false;
                    let mut t = p;
                    while let Some(&v) = it.next() {
                        if v == cnts[y] {
                            continue;
                        } else {
                            succ = true;
                            t -= v;
                            break;
                        }
                    }
                    if succ {
                        ans = ans.max((d - t).abs());
                    }
                }
            }
            sets[x].insert(cnts[x]);
        }
        ans
    }
}
