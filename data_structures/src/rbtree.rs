use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
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

impl<T> Index<Side> for [T] {
    type Output = T;

    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Left => &self[0],
            Side::Right => &self[1],
        }
    }
}

impl<T> IndexMut<Side> for [T] {
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

impl Color {
    #[inline]
    fn is_red(&self) -> bool {
        self == &Color::Red
    }

    #[inline]
    fn is_black(&self) -> bool {
        !self.is_red()
    }
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
    subtree_sz: [usize; 2],
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
            subtree_sz: [0, 0],
        }
    }

    fn get_side(&self, parent: *mut TreeNode<K, V>) -> Side {
        if unsafe { (*parent).child[0] } as *const _ == self as *const _ {
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
    size: usize,
}

impl<K, V> RedBlackTree<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord + Copy,
{
    pub fn new() -> Self {
        Self {
            root: ptr::null_mut(),
            size: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut node = self.root;
        unsafe {
            while !node.is_null() {
                let node_key = &(*node).key;
                if node_key == key {
                    return Some(&(*node).val);
                } else if node_key < &key {
                    node = (*node).child[Side::Right];
                } else {
                    node = (*node).child[Side::Left];
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut node = self.root;
        unsafe {
            while !node.is_null() {
                let node_key = &(*node).key;
                if node_key == key {
                    return Some(&mut (*node).val);
                } else if node_key < &key {
                    node = (*node).child[Side::Right];
                } else {
                    node = (*node).child[Side::Left];
                }
            }
        }
        None
    }

    pub fn kth_smallest_key(&self, mut kth: usize) -> Option<&K> {
        if kth > self.len() {
            return None;
        }
        let mut node = self.root;
        unsafe {
            while !node.is_null() {
                if (*node).subtree_sz[0] + 1 == kth {
                    return Some(&(*node).key);
                } else if (*node).subtree_sz[0] + 1 < kth {
                    kth -= (*node).subtree_sz[0] + 1;
                    node = (*node).child[Side::Right];
                } else {
                    node = (*node).child[Side::Left];
                }
            }
        }
        unreachable!();
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let mut parent: *mut TreeNode<K, V> = ptr::null_mut();
        let mut node = self.root;
        let mut side = Side::Left;
        unsafe {
            while !node.is_null() {
                let node_key = &(*node).key;
                if node_key == &key {
                    while !parent.is_null() {
                        if &(*parent).key > &key {
                            (*parent).subtree_sz[Side::Left] -= 1;
                        } else {
                            (*parent).subtree_sz[Side::Right] -= 1;
                        }
                        parent = (*parent).parent;
                    }
                    let old_val = std::mem::replace(&mut (*node).val, val);
                    return Some(old_val);
                } else if node_key < &key {
                    (*node).subtree_sz[Side::Right] += 1;
                    parent = node;
                    side = Side::Right;
                    node = (*node).child[Side::Right];
                } else {
                    (*node).subtree_sz[Side::Left] += 1;
                    parent = node;
                    side = Side::Left;
                    node = (*node).child[Side::Left];
                }
            }
        }
        self.size += 1;
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
                if (*parent).color.is_black() {
                    return;
                }
                // from now on parent.color is Red
                let grandparent = (*parent).parent;
                if grandparent.is_null() {
                    (*parent).color = Color::Black;
                    return;
                }
                let parent_side = (*parent).get_side(grandparent);
                let uncle = (*grandparent).child[parent_side.flip()];
                if uncle.is_null() || (*uncle).color.is_black() {
                    let node_side = (*node).get_side(parent);
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

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut parent = ptr::null_mut();
        let mut node = self.root;
        let mut side = Side::Left;
        unsafe {
            while !node.is_null() {
                let node_key = &(*node).key;
                if node_key == key {
                    break;
                } else if node_key < &key {
                    (*node).subtree_sz[Side::Right] -= 1;
                    parent = node;
                    side = Side::Right;
                    node = (*node).child[Side::Right];
                } else {
                    (*node).subtree_sz[Side::Left] -= 1;
                    parent = node;
                    side = Side::Left;
                    node = (*node).child[Side::Left];
                }
            }
        }
        if node.is_null() {
            unsafe {
                while !parent.is_null() {
                    if &(*parent).key > &key {
                        (*parent).subtree_sz[Side::Left] += 1;
                    } else {
                        (*parent).subtree_sz[Side::Right] += 1;
                    }
                    parent = (*parent).parent;
                }
            }

            return None;
        }
        self.size -= 1;
        unsafe {
            loop {
                if (*node).child[0].is_null() && (*node).child[1].is_null() {
                    if self.root == node {
                        self.root = ptr::null_mut();
                        return Some(Self::free_node(node));
                    } else if (*node).color.is_red() {
                        (*parent).child[side] = ptr::null_mut();
                        return Some(Self::free_node(node));
                    } else {
                        let v = Some(Self::free_node(node));
                        (*parent).child[side] = ptr::null_mut();
                        self.remove_helper(parent, side);
                        return v;
                    }
                } else if !(*node).child[0].is_null() && !(*node).child[1].is_null() {
                    (*node).subtree_sz[0] -= 1;
                    let mut biggest_less = (*node).child[0];
                    while !(*biggest_less).child[1].is_null() {
                        (*biggest_less).subtree_sz[1] -= 1;
                        biggest_less = (*biggest_less).child[1];
                    }
                    std::mem::swap(&mut (*node).key, &mut (*biggest_less).key);
                    std::mem::swap(&mut (*node).val, &mut (*biggest_less).val);
                    parent = (*biggest_less).parent;
                    side = (*biggest_less).get_side(parent);
                    node = biggest_less;
                } else if !(*node).child[0].is_null() {
                    if !parent.is_null() {
                        (*parent).child[side] = (*node).child[0];
                    }
                    (*(*node).child[0]).parent = parent;
                    (*(*node).child[0]).color = Color::Black;
                    if self.root == node {
                        self.root = (*node).child[0];
                    }
                    return Some(Self::free_node(node));
                } else {
                    if !parent.is_null() {
                        (*parent).child[side] = (*node).child[1];
                    }
                    (*(*node).child[1]).parent = parent;
                    (*(*node).child[1]).color = Color::Black;
                    if self.root == node {
                        self.root = (*node).child[1];
                    }
                    return Some(Self::free_node(node));
                }
            }
        }
    }

    fn remove_helper(&mut self, mut parent: *mut TreeNode<K, V>, mut side: Side) {
        unsafe {
            let mut sibling = (*parent).child[side.flip()];
            let mut distant_nephew = (*sibling).child[side.flip()];
            let mut close_nephew = (*sibling).child[side];
            loop {
                if (*sibling).color.is_red() {
                    self.rotate(parent, side);
                    (*parent).color = Color::Red;
                    (*sibling).color = Color::Black;
                    sibling = close_nephew;
                    distant_nephew = (*sibling).child[side.flip()];
                    close_nephew = (*sibling).child[side];
                } else if !distant_nephew.is_null() && (*distant_nephew).color.is_red() {
                    self.rotate(parent, side);
                    (*sibling).color = (*parent).color;
                    (*parent).color = Color::Black;
                    (*distant_nephew).color = Color::Black;
                    return;
                } else if !close_nephew.is_null() && (*close_nephew).color.is_red() {
                    self.rotate(sibling, side.flip());
                    (*sibling).color = Color::Red;
                    (*close_nephew).color = Color::Black;
                    distant_nephew = sibling;
                    sibling = close_nephew;
                    self.rotate(parent, side);
                    (*sibling).color = (*parent).color;
                    (*parent).color = Color::Black;
                    (*distant_nephew).color = Color::Black;
                    return;
                } else if (*parent).color.is_red() {
                    (*sibling).color = Color::Red;
                    (*parent).color = Color::Black;
                    return;
                } else {
                    (*sibling).color = Color::Red;
                    if (*parent).parent.is_null() {
                        break;
                    }
                    side = (*parent).get_side((*parent).parent);
                    parent = (*parent).parent;
                    sibling = (*parent).child[side.flip()];
                    distant_nephew = (*sibling).child[side.flip()];
                    close_nephew = (*sibling).child[side];
                }
            }
        }
    }

    fn free_node(node: *mut TreeNode<K, V>) -> V {
        unsafe {
            let boxed_node = Box::from_raw(node);
            boxed_node.val
        }
    }

    fn rotate(&mut self, node: *mut TreeNode<K, V>, side: Side) -> *mut TreeNode<K, V> {
        unsafe {
            let parent = (*node).parent;
            let flip_child = (*node).child[side.flip()];
            let grandchild = (*flip_child).child[side];
            (*node).child[side.flip()] = grandchild;
            if !grandchild.is_null() {
                (*node).subtree_sz[side.flip()] =
                    (*grandchild).subtree_sz[0] + (*grandchild).subtree_sz[1] + 1;
                (*grandchild).parent = node;
            } else {
                (*node).subtree_sz[side.flip()] = 0;
            }
            (*flip_child).child[side] = node;
            (*flip_child).parent = parent;
            (*flip_child).subtree_sz[side] = (*node).subtree_sz[0] + (*node).subtree_sz[1] + 1;
            (*node).parent = flip_child;
            if !parent.is_null() {
                let node_side = (*node).get_side(parent);
                (*parent).child[node_side] = flip_child;
            } else {
                self.root = flip_child;
            }
            flip_child
        }
    }
}

impl<K, V> Drop for RedBlackTree<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord,
{
    fn drop(&mut self) {
        if self.root.is_null() {
            return;
        }
        let mut stk = Vec::with_capacity(32.min(self.size));
        stk.push(self.root);
        while let Some(node) = stk.pop() {
            unsafe {
                if !(*node).child[0].is_null() {
                    stk.push((*node).child[0]);
                }
                if !(*node).child[1].is_null() {
                    stk.push((*node).child[1]);
                }
                drop(Box::from_raw(node));
            }
        }
    }
}

impl<K, V> RedBlackTree<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord + Copy + Debug + Display,
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
                        row.push(format!(
                            "{}{} {:?}",
                            (*node).key,
                            (*node).color,
                            (*node).subtree_sz
                        ));
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
