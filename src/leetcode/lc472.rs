#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let words = vec!["a", "ab", "abc", "cd", "abcd"];
        let ans = vec!["abcd"];
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(
                words.into_iter().map(|s| s.to_string()).collect()
            ),
            ans.into_iter()
                .map(|s: &str| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    nodes: Vec<Option<Rc<RefCell<Node>>>>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            nodes: vec![None; 26],
            is_end: false,
        }
    }
}

struct Trie {
    root: Rc<RefCell<Node>>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::new())),
        }
    }

    fn insert_word(&self, s: &[u8]) {
        let mut node = self.root.clone();
        for &c in s {
            let c = (c - b'a') as usize;
            {
                let mut inner = node.borrow_mut();
                if inner.nodes[c].is_none() {
                    inner.nodes[c] = Some(Rc::new(RefCell::new(Node::new())));
                }
            }
            node = node.clone().borrow().nodes[c].clone().unwrap();
        }
        node.borrow_mut().is_end = true;
    }

    fn search_word(&self, s: &[u8], idx: usize) -> bool {
        if idx == s.len() {
            return true;
        }
        let mut node = self.root.clone();
        for i in idx..s.len() {
            let c = (s[i] - b'a') as usize;
            if node.borrow().nodes[c].is_none() {
                return false;
            }
            node = node.clone().borrow().nodes[c].clone().unwrap();
            if node.borrow().is_end {
                let res = self.search_word(s, i + 1);
                if res {
                    return true;
                }
            }
        }
        return node.borrow().is_end;
    }
}

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by_key(|s| s.len());
        let mut ans = vec![];
        let trie = Trie::new();
        for word in words {
            let word = word.as_bytes();
            if word.len() != 0 {
                if trie.search_word(word, 0) {
                    ans.push(std::str::from_utf8(word).unwrap().to_string());
                } else {
                    trie.insert_word(word);
                }
            }
        }
        ans
    }
}
