#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let target = "abcdef";
        let words = ["abdef", "abc", "d", "def", "ef"];
        let costs = [100, 1, 1, 10, 5];
        let ans = 7;
        assert_eq!(
            Solution::minimum_cost(
                target.to_owned(),
                words.into_iter().map(|s| s.to_owned()).collect(),
                costs.to_vec()
            ),
            ans
        );
    }

    #[test]
    fn test1() {
        let target = "aaaa";
        let words = ["z","zz","zzz"];
        let costs = [1, 10, 100];
        let ans = -1;
        assert_eq!(
            Solution::minimum_cost(
                target.to_owned(),
                words.into_iter().map(|s| s.to_owned()).collect(),
                costs.to_vec()
            ),
            ans
        );
    }

    #[test]
    fn test2() {
        let target = "aaaa";
        let words = ["a","aa","aaa"];
        let costs = [1, 3, 2];
        let ans = 3;
        assert_eq!(
            Solution::minimum_cost(
                target.to_owned(),
                words.into_iter().map(|s| s.to_owned()).collect(),
                costs.to_vec()
            ),
            ans
        );
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

impl Solution {
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let target = target.as_bytes();
        let mut dp = vec![i32::MAX; target.len() + 1];
        let mut trie = Trie::new();
        let m = words.iter().map(|w| w.as_bytes().len()).sum::<usize>();
        let m = (m as f64).sqrt() as usize;
        let mut arr = vec![vec![]; target.len()];
        for (word, cost) in words.iter().zip(costs.iter()) {
            let word = word.as_bytes();
            if word.len() <= m {
                trie.insert(word, *cost);
            } else {
                let mut prefix = vec![0; word.len()];
                for i in 1..word.len() {
                    let mut j = i;
                    while j != 0 && word[prefix[j - 1]] != word[i] {
                        j = prefix[j - 1];
                    }
                    if word[j] == word[i] {
                        prefix[i] = if j == 0 { 0 } else { prefix[j - 1] } + 1;
                    }
                }
                let mut i = 0;
                let mut j = 0;
                while i < target.len() {
                    if target[i] != word[j] {
                        if j == 0 {
                            i += 1;
                            j = 0;
                        } else {
                            j = prefix[j - 1];
                        }
                    } else if j == word.len() - 1 {
                        arr[i + 1 - word.len()].push((word.len(), *cost));
                        j = if j > 0 { prefix[j - 1] } else { 0 };
                    } else {
                        i += 1;
                        j += 1;
                    }
                }
            }
        }
        dp[0] = 0;
        for i in 0..target.len() {
            if dp[i] == i32::MAX {
                continue;
            }
            for v in &arr[i] {
                if i + v.0 < dp.len() {
                    dp[i + v.0] = dp[i + v.0].min(dp[i] + v.1);
                }
            }
            let mut node: *mut TrieNode = &mut trie.root as *mut _;
            let mut j = i;
            while j < target.len()
                && !(unsafe { &*node }).nxt[(target[j] - b'a') as usize].is_null()
            {
                node = (unsafe { &*node }).nxt[(target[j] - b'a') as usize];
                if let Some(t) = (unsafe { &*node }).end {
                    dp[j + 1] = dp[j + 1].min(dp[i] + t);
                }
                j += 1;
            }
        }

        if dp[target.len()] == i32::MAX {
            -1
        } else {
            dp[target.len()]
        }
    }
}

struct Trie {
    root: TrieNode,
}

struct TrieNode {
    nxt: [*mut TrieNode; 26],
    end: Option<i32>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            nxt: [std::ptr::null_mut(); 26],
            end: None,
        }
    }
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &[u8], cost: i32) {
        let mut node: *mut TrieNode = &mut self.root as *mut _;
        for c in word {
            let k = (*c - b'a') as usize;
            let n = (unsafe { &*node }).nxt[k];
            if n.is_null() {
                let n = Box::new(TrieNode::new());
                (unsafe { &mut *node }).nxt[k] = Box::leak(n) as *mut _;
            }
            node = (unsafe { &*node }).nxt[k];
        }
        if let Some(t) = (unsafe { &mut *node }).end.as_mut() {
            *t = cost.min(*t);
        } else {
            (unsafe { &mut *node }).end = Some(cost);
        }
    }
}
