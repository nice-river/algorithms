use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};

#[derive(Debug, Default)]
struct Node<T: Ord + Clone> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
    down: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord + Clone> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
            down: None,
        }
    }
}

pub struct SkipList<T: Ord + Default + Clone> {
    cur_max_layer: usize,
    layers: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug + Ord + Default + Clone> SkipList<T> {
    pub fn new(max_layer: usize) -> Self {
        let mut layers = Vec::with_capacity(max_layer);
        layers.push(Rc::new(RefCell::new(Node::default())));
        for i in 1..max_layer {
            let mut node = Node::default();
            node.down = Some(layers[i - 1].clone());
            layers.push(Rc::new(RefCell::new(node)));
            layers.last_mut().unwrap();
        }
        Self {
            cur_max_layer: 0,
            layers,
        }
    }

    pub fn insert(&mut self, val: T) {
        let mut cur_layer = self.cur_max_layer;
        let mut node = self.layers[cur_layer].clone();
        let mut pre_nodes = self.layers.clone();
        let mut cmp_times = 0;
        loop {
            let mut next_node = RefCell::borrow(node.as_ref()).next.clone();
            while let Some(next) = next_node {
                let x = RefCell::borrow(&next);
                cmp_times += 1;
                match x.val.cmp(&val) {
                    Ordering::Less => {
                        node = next.clone();
                        next_node = RefCell::borrow(node.as_ref()).next.clone();
                    }
                    _ => break,
                }
            }
            let down = RefCell::borrow(node.as_ref()).down.clone();
            if let Some(down) = down {
                pre_nodes[cur_layer] = node.clone();
                node = down;
                cur_layer -= 1;
            } else {
                break;
            }
        }
        node = self.append_after_node(node.clone(), val.clone());
        for i in 1..self.layers.len() {
            if rand::random() {
                let new_node = self.append_after_node(pre_nodes[i].clone(), val.clone());
                let mut new_node_mut_ref = RefCell::borrow_mut(&new_node);
                new_node_mut_ref.down = Some(node);
                node = new_node.clone();
                self.cur_max_layer = self.cur_max_layer.max(i);
            } else {
                break;
            }
        }
    }

    fn append_after_node(&mut self, node: Rc<RefCell<Node<T>>>, val: T) -> Rc<RefCell<Node<T>>> {
        let mut ref_node = RefCell::borrow_mut(&node);
        let mut new_node = Node::new(val);
        new_node.next = ref_node.next.take();
        new_node.prev = Some(Rc::downgrade(&node));
        let new_node = Rc::new(RefCell::new(new_node));
        ref_node.next = Some(new_node.clone());
        if let Some(x) = new_node.borrow_mut().next.as_ref() {
            x.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        }
        new_node
    }

    pub fn search(&self, val: T) -> bool {
        let mut cur_layer = self.cur_max_layer;
        let mut node = self.layers[cur_layer].clone();
        loop {
            let mut next_node = RefCell::borrow(node.as_ref()).next.clone();
            while let Some(next) = next_node {
                let x = RefCell::borrow(&next);
                match x.val.cmp(&val) {
                    Ordering::Less => {
                        node = next.clone();
                        next_node = RefCell::borrow(node.as_ref()).next.clone();
                    }
                    Ordering::Equal => {
                        return true;
                    }
                    Ordering::Greater => {
                        break;
                    }
                }
            }
            let down = RefCell::borrow(node.as_ref()).down.clone();
            if let Some(down) = down {
                node = down;
            } else {
                break;
            }
        }
        false
    }

    pub fn remove(&mut self, val: T) -> bool {
        let mut cur_layer = self.cur_max_layer;
        let mut node = self.layers[cur_layer].clone();
        loop {
            let mut next_node = RefCell::borrow(node.as_ref()).next.clone();
            while let Some(next) = next_node {
                let x = RefCell::borrow(&next);
                match x.val.cmp(&val) {
                    Ordering::Less => {
                        node = next.clone();
                        next_node = RefCell::borrow(node.as_ref()).next.clone();
                    }
                    Ordering::Equal => {
                        drop(x);
                        let mut node = next.clone();
                        loop {
                            let mut x = RefCell::borrow_mut(&node);
                            if let Some(t) = x.next.as_ref() {
                                t.borrow_mut().prev = x.prev.clone();
                            }
                            if let Some(t) = x.prev.as_ref() {
                                if let Some(t) = t.upgrade() {
                                    t.borrow_mut().next = x.next.clone();
                                }
                            }
                            let down = x.down.take();
                            drop(x);
                            if let Some(down_node) = down {
                                node = down_node.clone();
                            } else {
                                break;
                            }
                        }
                        return true;
                    }
                    Ordering::Greater => {
                        break;
                    }
                }
            }
            let down = RefCell::borrow(node.as_ref()).down.clone();
            if let Some(down) = down {
                node = down;
            } else {
                break;
            }
        }
        false
    }
}

impl<T: Debug + Clone + Ord + Default> Debug for SkipList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let default = Rc::new(RefCell::new(Node::default()));
        for i in (0..=1).rev() {
            write!(
                f,
                "head({:p})(down: {:p})",
                self.layers[i],
                self.layers[i]
                    .as_ref()
                    .borrow()
                    .down
                    .as_ref()
                    .unwrap_or(&default)
                    .clone()
            )?;
            let mut node = self.layers[i].clone();
            let mut next_node = RefCell::borrow(node.as_ref()).next.clone();
            while let Some(x) = next_node {
                write!(f, " -> ")?;
                write!(
                    f,
                    "{:p}(down: {:p})",
                    x.as_ref(),
                    x.as_ref()
                        .borrow()
                        .down
                        .as_ref()
                        .unwrap_or(&default)
                        .clone()
                )?;
                node = x.clone();
                next_node = RefCell::borrow(node.as_ref()).next.clone();
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl<T: Ord + Default + Clone> Drop for SkipList<T> {
    fn drop(&mut self) {
        for i in (0..=self.cur_max_layer).rev() {
            let mut node = self.layers[i].clone();
            loop {
                let x = node.borrow_mut().next.take();
                if x.is_some() {
                    node = x.unwrap();
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::time::Instant;

    #[test]
    fn test_remove() {
        let mut skip_list: SkipList<i32> = SkipList::new(4);
        assert!(!skip_list.search(0));
        skip_list.insert(0);
        assert!(skip_list.search(0));
        assert!(skip_list.remove(0));
        assert!(!skip_list.remove(0));
    }

    #[test]
    fn test_skip_list() {
        let mut skip_list: SkipList<i32> = SkipList::new(32);
        let start = Instant::now();
        let n = 10000000;
        for i in 0..n {
            skip_list.insert(i);
        }

        println!("insert elapsed: {:?}", start.elapsed());

        let start = Instant::now();
        let mut remove_set = HashSet::new();
        for i in 0..n {
            if i % 2 == 0 {
                remove_set.insert(i);
                skip_list.remove(i);
            }
        }
        for i in 0..n {
            assert!(!skip_list.search(i) ^ !remove_set.contains(&i))
        }
        println!("remove and search elapsed: {:?}", start.elapsed());
    }

    #[test]
    fn test_btree_map() {
        let mut map = std::collections::BTreeSet::new();
        let start = Instant::now();
        let n = 10000000;
        for i in 0..n {
            map.insert(i);
        }

        println!("insert elapsed: {:?}", start.elapsed());

        let start = Instant::now();
        let mut remove_set = HashSet::new();
        for i in 0..n {
            if i % 2 == 0 {
                remove_set.insert(i);
                map.remove(&i);
            }
        }
        for i in 0..n {
            assert!(!map.get(&i).is_some() ^ !remove_set.contains(&i))
        }
        println!("remove and search elapsed: {:?}", start.elapsed());
    }
}
