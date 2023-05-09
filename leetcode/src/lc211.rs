struct WordDictionary {
    is_end: bool,
    children: Vec<Option<Box<WordDictionary>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            is_end: false,
            children: (0..26).map(|_| None).collect::<Vec<_>>(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut *self;
        for s in word.as_bytes() {
            let k = (s - b'a') as usize;
            if node.children[k].is_none() {
                node.children[k] = Some(Box::new(WordDictionary::new()));
            }
            node = &mut (**node.children[k].as_mut().unwrap());
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        let mut stk = vec![(self, 0)];

        while !stk.is_empty() {
            let (node, idx) = stk.pop().unwrap();
            if idx == word.len() {
                if node.is_end {
                    return true;
                }
                continue;
            }
            if word[idx] == b'.' {
                for ch in node.children.iter() {
                    if let Some(ch) = ch {
                        stk.push((&**ch, idx + 1));
                    }
                }
            } else {
                let k = (word[idx] - b'a') as usize;
                if let Some(ch) = node.children[k].as_ref() {
                    stk.push((&**ch, idx + 1));
                }
            }
        }
        false
    }
}
