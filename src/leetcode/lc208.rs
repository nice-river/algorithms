#[derive(Default)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26],
}

struct Trie {
    node: Option<Box<Node>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            node: Some(Box::new(Node::default())),
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut node =  self.node.as_mut();
        for &ch in word.as_bytes() {
            let k = (ch - b'a') as usize;
            if let Some(x) = node {
                if x.children[k].is_none() {
                    x.children[k] = Some(Box::new(Node::default()));
                }
                node = x.children[k].as_mut();
            }
        }
        node.unwrap().end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut node =  self.node.as_ref();
        for &ch in word.as_bytes() {
            let k = (ch - b'a') as usize;
            if let Some(x) = node {
                if x.children[k].is_none() {
                    return false;
                }
                node = x.children[k].as_ref();
            }
        }
        node.unwrap().end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut node =  self.node.as_ref();
        for &ch in prefix.as_bytes() {
            let k = (ch - b'a') as usize;
            if let Some(x) = node {
                if x.children[k].is_none() {
                    return false;
                }
                node = x.children[k].as_ref();
            }
        }
        true
    }
}
