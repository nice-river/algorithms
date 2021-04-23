use std::rc::Rc;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::borrow::Borrow;
use rand::random;

#[derive(Debug, Default)]
struct Node<T: Ord + Clone> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    down: Option<Rc<RefCell<Node<T>>>>,
    span: usize,
}

impl<T: Ord + Clone> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            down: None,
            span: 0,
        }
    }
}


pub struct SkipList<T: Ord + Default + Clone> {
    cur_max_layer: usize,
    layers: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord + Default + Clone> SkipList<T> {
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
            layers
        }
    }

    pub fn insert(&mut self, val: T) {
        let mut cur_layer = self.cur_max_layer;
        let mut node = self.layers[cur_layer].clone();
        let mut pre_nodes = self.layers.clone();
        loop {
            let mut next_node = RefCell::borrow(node.as_ref()).next.clone();
            while let Some(next) = next_node {
                let x = RefCell::borrow(&next);
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
        self.append_after_node(node.clone(), val.clone());
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
        let mut node = RefCell::borrow_mut(&node);
        let mut new_node = Node::new(val);
        new_node.next = node.next.clone();
        let new_node = Rc::new(RefCell::new(new_node));
        node.next = Some(new_node.clone());
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut skip_list: SkipList<i32> = SkipList::new(4);
    }

    #[test]
    fn test_search() {
        let mut skip_list: SkipList<i32> = SkipList::new(4);
        let n = 100;
        for i in (0..n).step_by(2) {
            skip_list.insert(i);
        }
        for i in (0..n).step_by(2) {
            assert!(skip_list.search(i));
        }
        for i in (1..n).step_by(2) {
            assert!(!skip_list.search(i));
        }
    }
}