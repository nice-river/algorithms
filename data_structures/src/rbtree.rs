use std::{
    fmt::{Debug, Display},
    ptr,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left = 0,
    Right = 1,
}

impl Side {
    #[inline]
    fn flip(&self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<K, V> std::ops::Index<Side> for [*mut TreeNode<K, V>]
where
    K: PartialEq + Eq + PartialOrd + Ord,
{
    type Output = *mut TreeNode<K, V>;

    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Left => &self[0],
            Side::Right => &self[1],
        }
    }
}

impl<K, V> std::ops::IndexMut<Side> for [*mut TreeNode<K, V>]
where
    K: PartialEq + Eq + PartialOrd + Ord,
{
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::Left => &mut self[0],
            Side::Right => &mut self[1],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Black => write!(f, "B"),
        }
    }
}

struct TreeNode<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord,
{
    key: K,
    val: V,
    color: Color,
    parent: *mut TreeNode<K, V>,
    child: [*mut TreeNode<K, V>; 2],
}

impl<K, V> TreeNode<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord,
{
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            color: Color::Red,
            parent: ptr::null_mut(),
            child: [ptr::null_mut(); 2],
        }
    }

    fn get_side(&self, parent: &TreeNode<K, V>) -> Side {
        if (*parent).child[0] as *const _ == self as *const _ {
            Side::Left
        } else {
            Side::Right
        }
    }
}

pub struct RedBlackTree<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord,
{
    root: *mut TreeNode<K, V>,
}

impl<K, V> RedBlackTree<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord + Copy,
{
    pub fn new() -> Self {
        Self {
            root: ptr::null_mut(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let mut parent = ptr::null_mut();
        let mut node = self.root;
        let mut side = Side::Left;
        while !node.is_null() {
            let node_key = unsafe { &(*node).key };
            if node_key == &key {
                let old_val = std::mem::replace(unsafe { &mut (*node).val }, val);
                return Some(old_val);
            } else if node_key < &key {
                parent = node;
                side = Side::Right;
                node = unsafe { (*node).child[Side::Right] };
            } else {
                parent = node;
                side = Side::Left;
                node = unsafe { (*node).child[Side::Left] };
            }
        }
        let boxed_node = Box::new(TreeNode::new(key, val));
        node = Box::leak(boxed_node) as *mut _;
        self.insert_helper(node, parent, side);
        None
    }

    fn insert_helper(
        &mut self,
        mut node: *mut TreeNode<K, V>,
        mut parent: *mut TreeNode<K, V>,
        side: Side,
    ) {
        if parent.is_null() {
            self.root = node;
            return;
        }
        unsafe {
            (*node).color = Color::Red;
            (*node).parent = parent;
            (*parent).child[side] = node;
            loop {
                if (*parent).color == Color::Black {
                    return;
                }
                // from now on parent.color is Red
                let grandparent = (*parent).parent;
                if grandparent.is_null() {
                    (*parent).color = Color::Black;
                    return;
                }
                let parent_side = (*parent).get_side(&*grandparent);
                let uncle = (*grandparent).child[parent_side.flip()];
                if uncle.is_null() || (*uncle).color == Color::Black {
                    let node_side = (*node).get_side(&*parent);
                    if node_side != parent_side {
                        self.rotate(parent, parent_side);
                        parent = (*grandparent).child[parent_side];
                    }
                    self.rotate(grandparent, parent_side.flip());
                    (*parent).color = Color::Black;
                    (*grandparent).color = Color::Red;
                    return;
                }
                (*parent).color = Color::Black;
                (*uncle).color = Color::Black;
                (*grandparent).color = Color::Red;
                node = grandparent;
                parent = (*node).parent;
                if parent.is_null() {
                    break;
                }
            }
        }
    }

    #[inline]
    fn rotate(&mut self, node: *mut TreeNode<K, V>, side: Side) -> *mut TreeNode<K, V> {
        unsafe {
            let parent = (*node).parent;
            let flip_child = (*node).child[side.flip()];
            let grandchild = (*flip_child).child[side];
            (*node).child[side.flip()] = grandchild;
            if !grandchild.is_null() {
                (*grandchild).parent = node;
            }
            (*flip_child).child[side] = node;
            (*flip_child).parent = parent;
            (*node).parent = flip_child;
            if !parent.is_null() {
                let node_side = (*node).get_side(&*parent);
                (*parent).child[node_side] = flip_child;
            } else {
                self.root = flip_child;
            }
            flip_child
        }
    }
}

impl<K, V> RedBlackTree<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord + Copy + Display,
{
    pub fn bfs(&self) -> Vec<Vec<String>> {
        use std::collections::VecDeque;
        let mut que = [VecDeque::new(), VecDeque::new()];
        que[0].push_back(self.root);
        let mut cur = 0;
        let mut res = vec![];
        while !que[cur].is_empty() {
            let nxt = cur ^ 1;
            let mut row = vec![];
            let mut all_null = true;
            while let Some(node) = que[cur].pop_front() {
                if node.is_null() {
                    que[nxt].push_back(ptr::null_mut());
                    que[nxt].push_back(ptr::null_mut());
                    row.push("null".to_owned());
                } else {
                    all_null = false;
                    unsafe {
                        row.push(format!("{}{}", (*node).key, (*node).color));
                        que[nxt].push_back((*node).child[0]);
                        que[nxt].push_back((*node).child[1]);
                    }
                }
            }
            if all_null {
                break;
            }
            res.push(row);
            cur ^= 1;
        }
        res
    }

    pub fn debug(&self) -> Vec<K> {
        let mut res = vec![];
        Self::dfs(self.root, &mut res);
        res
    }

    fn dfs(node: *mut TreeNode<K, V>, res: &mut Vec<K>) {
        if node.is_null() {
            return;
        }
        unsafe {
            Self::dfs((*node).child[0], res);
            res.push((*node).key);
            Self::dfs((*node).child[1], res);
        }
    }
}
