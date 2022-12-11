/// red black tree implementation based on [wikipedia](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree)
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Bound, Index, IndexMut, RangeBounds},
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
    K: Ord,
{
    key: K,
    val: V,
    color: Color,
    parent: *mut TreeNode<K, V>,
    child: [*mut TreeNode<K, V>; 2],
    subtree_sz: [usize; 2], // size of subtrees, used to find the kth_smallest elment
    next: *mut TreeNode<K, V>, // the next treenode whick key is greater
    prev: *mut TreeNode<K, V>, // the prev threenode whick key is lesser
}

impl<K, V> TreeNode<K, V>
where
    K: Ord,
{
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            color: Color::Red,
            parent: ptr::null_mut(),
            child: [ptr::null_mut(); 2],
            subtree_sz: [0, 0],
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
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

pub struct RBTreeMap<K, V>
where
    K: Ord,
{
    root: *mut TreeNode<K, V>,
    size: usize,
}

impl<K, V> RBTreeMap<K, V>
where
    K: Ord,
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

    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut node = self.root;
        unsafe {
            while !node.is_null() {
                let node_key = &(*node).key;
                if node_key.borrow() == key {
                    return Some(&(*node).val);
                } else if node_key.borrow() < key {
                    node = (*node).child[Side::Right];
                } else {
                    node = (*node).child[Side::Left];
                }
            }
        }
        None
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut node = self.root;
        unsafe {
            while !node.is_null() {
                let node_key = &(*node).key;
                if node_key.borrow() == key {
                    return Some(&mut (*node).val);
                } else if node_key.borrow() < key {
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
                        // don't need to add new node, rollback the subtree_sz
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
        unsafe {
            // add new node, update next and prev pointer
            let boxed_node = Box::new(TreeNode::new(key, val));
            node = Box::leak(boxed_node) as *mut _;
            if parent.is_null() {
                self.root = node;
                return None;
            }
            (*node).color = Color::Red;
            (*node).parent = parent;
            (*parent).child[side] = node;
            match side {
                Side::Left => {
                    if !(*parent).prev.is_null() {
                        (*(*parent).prev).next = node;
                    }
                    (*node).prev = (*parent).prev;
                    (*node).next = parent;
                    (*parent).prev = node;
                }
                Side::Right => {
                    if !(*parent).next.is_null() {
                        (*(*parent).next).prev = node;
                    }
                    (*node).next = (*parent).next;
                    (*node).prev = parent;
                    (*parent).next = node;
                }
            }
        }

        self.insert_helper(node, parent);
        None
    }

    fn insert_helper(&mut self, mut node: *mut TreeNode<K, V>, mut parent: *mut TreeNode<K, V>) {
        // the wikipedia insertion cases
        unsafe {
            loop {
                if (*parent).color.is_black() {
                    // Case I1
                    return;
                }
                // from now on parent.color is Red
                let grandparent = (*parent).parent;
                if grandparent.is_null() {
                    // Case I4
                    (*parent).color = Color::Black;
                    return;
                }
                let parent_side = (*parent).get_side(grandparent);
                let uncle = (*grandparent).child[parent_side.flip()];
                if uncle.is_null() || (*uncle).color.is_black() {
                    // Case I5
                    let node_side = (*node).get_side(parent);
                    if node_side != parent_side {
                        self.rotate(parent, parent_side);
                        parent = (*grandparent).child[parent_side];
                    }
                    // Case I6
                    self.rotate(grandparent, parent_side.flip());
                    (*parent).color = Color::Black;
                    (*grandparent).color = Color::Red;
                    return;
                }
                // Case I2
                (*parent).color = Color::Black;
                (*uncle).color = Color::Black;
                (*grandparent).color = Color::Red;
                node = grandparent;
                parent = (*node).parent;
                if parent.is_null() {
                    // Case I3
                    break;
                }
            }
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut parent = ptr::null_mut();
        let mut node = self.root;
        let mut side = Side::Left;
        unsafe {
            while !node.is_null() {
                let node_key = &(*node).key;
                if node_key.borrow() == key {
                    break;
                } else if node_key.borrow() < key {
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
                    // can't find the node, rollback the subtree_sz
                    if (*parent).key.borrow() > key {
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
                    // do not have any children
                    if self.root == node {
                        // the node is the root, just delete it
                        self.root = ptr::null_mut();
                        return Some(Self::free_node(node));
                    } else if (*node).color.is_red() {
                        // color is red, delete it would not break the rule
                        (*parent).child[side] = ptr::null_mut();
                        return Some(Self::free_node(node));
                    } else {
                        // node's color is balck and do not have any children
                        // this branch is the complicated case
                        let v = Some(Self::free_node(node));
                        (*parent).child[side] = ptr::null_mut();
                        self.remove_helper(parent, side);
                        return v;
                    }
                } else if !(*node).child[0].is_null() && !(*node).child[1].is_null() {
                    // have 2 children
                    // swap this node and the biggest node that smaller than this node(i.e. node.prev, and node.prev must exist, since the left child is not null),
                    // the node will be removed in next iteration
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
                    // in the next iteration, the node will be deleted in other branches
                    node = biggest_less;
                } else if !(*node).child[0].is_null() {
                    // if it has only one child, the child must be red color, otherwise it will violate requirement 4
                    // then this node must be red color, otherwise it will violate requirement 3
                    // so just simply remove this node, and make the child black is good enough
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
                    // the same as above branch
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
        // the complicated wikipedia remove cases
        unsafe {
            let mut sibling = (*parent).child[side.flip()];
            let mut distant_nephew = (*sibling).child[side.flip()];
            let mut close_nephew = (*sibling).child[side];
            loop {
                if (*sibling).color.is_red() {
                    // Case D3
                    self.rotate(parent, side);
                    (*parent).color = Color::Red;
                    (*sibling).color = Color::Black;
                    sibling = close_nephew;
                    distant_nephew = (*sibling).child[side.flip()];
                    close_nephew = (*sibling).child[side];
                } else if !distant_nephew.is_null() && (*distant_nephew).color.is_red() {
                    // Case D6
                    self.rotate(parent, side);
                    (*sibling).color = (*parent).color;
                    (*parent).color = Color::Black;
                    (*distant_nephew).color = Color::Black;
                    return;
                } else if !close_nephew.is_null() && (*close_nephew).color.is_red() {
                    // Case D5
                    self.rotate(sibling, side.flip());
                    (*sibling).color = Color::Red;
                    (*close_nephew).color = Color::Black;
                    distant_nephew = sibling;
                    sibling = close_nephew;
                    // Case D6
                    self.rotate(parent, side);
                    (*sibling).color = (*parent).color;
                    (*parent).color = Color::Black;
                    (*distant_nephew).color = Color::Black;
                    return;
                } else if (*parent).color.is_red() {
                    // Case D4
                    (*sibling).color = Color::Red;
                    (*parent).color = Color::Black;
                    return;
                } else {
                    (*sibling).color = Color::Red;
                    if (*parent).parent.is_null() {
                        // Case D2
                        break;
                    }
                    // Case D1
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
        // free memory and update next, prev pointer
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            if !prev.is_null() {
                (*prev).next = next;
            }
            if !next.is_null() {
                (*next).prev = prev;
            }
            let boxed_node = Box::from_raw(node);
            boxed_node.val
        }
    }

    fn rotate(&mut self, node: *mut TreeNode<K, V>, side: Side) -> *mut TreeNode<K, V> {
        // wikipedia, RoateDirRoot function
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

    pub fn iter(&self) -> RBTreeMapIter<K, V> {
        let head = self.find_head(Bound::Unbounded);
        let tail = self.find_tail(Bound::Unbounded);
        RBTreeMapIter {
            head,
            tail,
            _phantom: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> RBTreeMapIterMut<K, V> {
        let head = self.find_head(Bound::Unbounded);
        let tail = self.find_tail(Bound::Unbounded);
        RBTreeMapIterMut {
            head,
            tail,
            _tree: self,
        }
    }

    pub fn range<Q, R>(&self, range: R) -> RBTreeMapIter<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
        R: RangeBounds<Q>,
    {
        let head = self.find_head(range.start_bound());
        let tail = self.find_tail(range.end_bound());
        RBTreeMapIter {
            head,
            tail,
            _phantom: PhantomData,
        }
    }

    pub fn range_mut<Q, R>(&mut self, range: R) -> RBTreeMapIterMut<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
        R: RangeBounds<Q>,
    {
        let head = self.find_head(range.start_bound());
        let tail = self.find_tail(range.end_bound());
        RBTreeMapIterMut {
            head,
            tail,
            _tree: self,
        }
    }

    fn find_head<Q>(&self, bound: Bound<&Q>) -> *mut TreeNode<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        // find the first node's key >= bound(based on the bound enum)
        let mut node = self.root;
        if node.is_null() {
            return node;
        }
        unsafe {
            match bound {
                Bound::Unbounded => {
                    while !(*node).child[0].is_null() {
                        node = (*node).child[0];
                    }
                }
                Bound::Included(bound) => {
                    let mut ans = ptr::null_mut();
                    while !node.is_null() {
                        match (*node).key.borrow().cmp(bound) {
                            std::cmp::Ordering::Equal => {
                                ans = node;
                                node = (*node).child[Side::Left];
                            }
                            std::cmp::Ordering::Less => {
                                node = (*node).child[Side::Right];
                            }
                            std::cmp::Ordering::Greater => {
                                ans = node;
                                node = (*node).child[Side::Left];
                            }
                        }
                    }
                    node = ans;
                }
                Bound::Excluded(bound) => {
                    let mut ans = ptr::null_mut();
                    while !node.is_null() {
                        match (*node).key.borrow().cmp(bound) {
                            std::cmp::Ordering::Equal => {
                                node = (*node).child[Side::Right];
                            }
                            std::cmp::Ordering::Less => {
                                node = (*node).child[Side::Right];
                            }
                            std::cmp::Ordering::Greater => {
                                ans = node;
                                node = (*node).child[Side::Left];
                            }
                        }
                    }
                    node = ans;
                }
            }
        }
        node
    }

    fn find_tail<Q>(&self, bound: Bound<&Q>) -> *mut TreeNode<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        // find the last node's key <= bound(based on the bound enum)
        let mut node = self.root;
        if node.is_null() {
            return node;
        }
        unsafe {
            match bound {
                Bound::Unbounded => {
                    while !(*node).child[Side::Right].is_null() {
                        node = (*node).child[Side::Right];
                    }
                }
                Bound::Included(bound) => {
                    let mut ans = ptr::null_mut();
                    while !node.is_null() {
                        match (*node).key.borrow().cmp(bound) {
                            std::cmp::Ordering::Equal => {
                                ans = node;
                                node = (*node).child[Side::Right];
                            }
                            std::cmp::Ordering::Less => {
                                ans = node;
                                node = (*node).child[Side::Right];
                            }
                            std::cmp::Ordering::Greater => {
                                node = (*node).child[Side::Left];
                            }
                        }
                    }
                    node = ans;
                }
                Bound::Excluded(bound) => {
                    let mut ans = ptr::null_mut();
                    while !node.is_null() {
                        match (*node).key.borrow().cmp(bound) {
                            std::cmp::Ordering::Equal => {
                                node = (*node).child[Side::Left];
                            }
                            std::cmp::Ordering::Less => {
                                ans = node;
                                node = (*node).child[Side::Right];
                            }
                            std::cmp::Ordering::Greater => {
                                node = (*node).child[Side::Left];
                            }
                        }
                    }
                    node = ans;
                }
            }
        }
        node
    }
}

pub struct RBTreeMapIter<'a, K, V>
where
    K: Ord,
{
    head: *mut TreeNode<K, V>,
    tail: *mut TreeNode<K, V>,
    _phantom: PhantomData<&'a RBTreeMap<K, V>>,
}

impl<'a, K, V> Iterator for RBTreeMapIter<'a, K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.head.is_null() || (*self.head).prev == self.tail {
                None
            } else {
                let ret = Some((&(*self.head).key, &(*self.head).val));
                self.head = (*self.head).next;
                ret
            }
        }
    }
}

impl<'a, K, V> DoubleEndedIterator for RBTreeMapIter<'a, K, V>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.tail.is_null() || (*self.tail).next == self.head {
                None
            } else {
                let ret = Some((&(*self.tail).key, &(*self.tail).val));
                self.tail = (*self.tail).prev;
                ret
            }
        }
    }
}

pub struct RBTreeMapIterMut<'a, K, V>
where
    K: Ord,
{
    head: *mut TreeNode<K, V>,
    tail: *mut TreeNode<K, V>,
    _tree: &'a mut RBTreeMap<K, V>,
}

impl<'a, K, V> Iterator for RBTreeMapIterMut<'a, K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.head.is_null() || (*self.head).prev == self.tail {
                None
            } else {
                let ret = Some((&(*self.head).key, &mut (*self.head).val));
                self.head = (*self.head).next;
                ret
            }
        }
    }
}

impl<'a, K, V> DoubleEndedIterator for RBTreeMapIterMut<'a, K, V>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.tail.is_null() || (*self.tail).next == self.head {
                None
            } else {
                let ret = Some((&(*self.tail).key, &mut (*self.tail).val));
                self.tail = (*self.tail).prev;
                ret
            }
        }
    }
}

impl<K, V> Drop for RBTreeMap<K, V>
where
    K: Ord,
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

pub struct RBTreeMapIntoIter<K, V>
where
    K: Ord,
{
    head: *mut TreeNode<K, V>,
    tail: *mut TreeNode<K, V>,
    tree: RBTreeMap<K, V>,
}

impl<K, V> Iterator for RBTreeMapIntoIter<K, V>
where
    K: Ord,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
            self.tree.root = ptr::null_mut();
            None
        } else {
            unsafe {
                let boxed_node = Box::from_raw(self.head);
                self.head = boxed_node.next;
                if !self.head.is_null() {
                    (*self.head).prev = ptr::null_mut();
                }
                let key = boxed_node.key;
                let val = boxed_node.val;
                Some((key, val))
            }
        }
    }
}

impl<K, V> DoubleEndedIterator for RBTreeMapIntoIter<K, V>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.tail.is_null() {
            self.tree.root = ptr::null_mut();
            None
        } else {
            unsafe {
                let boxed_node = Box::from_raw(self.tail);
                self.tail = boxed_node.prev;
                if !self.tail.is_null() {
                    (*self.tail).next = ptr::null_mut();
                }
                let key = boxed_node.key;
                let val = boxed_node.val;
                Some((key, val))
            }
        }
    }
}

impl<K, V> IntoIterator for RBTreeMap<K, V>
where
    K: Ord,
{
    type Item = (K, V);
    type IntoIter = RBTreeMapIntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let head = self.find_head(Bound::Unbounded);
        let tail = self.find_tail(Bound::Unbounded);
        RBTreeMapIntoIter {
            head,
            tail,
            tree: self,
        }
    }
}

impl<K, V> Drop for RBTreeMapIntoIter<K, V>
where
    K: Ord,
{
    fn drop(&mut self) {
        while !self.head.is_null() {
            // free the rest nodes memory
            unsafe {
                let boxed_node = Box::from_raw(self.head);
                self.head = boxed_node.next;
                drop(boxed_node);
            }
        }
        self.tree.root = ptr::null_mut();
    }
}

// used for debug
impl<K, V> RBTreeMap<K, V>
where
    K: Ord + Copy + Debug + Display,
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
        dbg!(&res);
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

pub struct RBTreeSet<K>
where
    K: Ord,
{
    map: RBTreeMap<K, ()>,
}

impl<K> RBTreeSet<K>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            map: RBTreeMap::new(),
        }
    }

    #[inline]
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map.contains_key(key)
    }

    pub fn insert(&mut self, key: K) -> bool {
        self.map.insert(key, ()).is_none()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map.remove(key).is_some()
    }

    pub fn iter(&self) -> RBTreeSetIter<K> {
        RBTreeSetIter {
            map_iter: self.map.iter(),
        }
    }

    pub fn range<Q, R>(&self, range: R) -> RBTreeSetIter<K>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
        R: RangeBounds<Q>,
    {
        RBTreeSetIter {
            map_iter: self.map.range(range),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn union<'a>(&'a self, other: &'a RBTreeSet<K>) -> RBTreeSetUnion<'a, K> {
        RBTreeSetUnion::new(self.iter(), other.iter())
    }

    // TODO implement difference(), symmetrice_difference(), insertsection()
}

pub struct RBTreeSetIter<'a, K>
where
    K: Ord,
{
    map_iter: RBTreeMapIter<'a, K, ()>,
}

impl<'a, K> Iterator for RBTreeSetIter<'a, K>
where
    K: Ord,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter.next().map(|(k, _)| k)
    }
}

impl<'a, K> DoubleEndedIterator for RBTreeSetIter<'a, K>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.map_iter.next_back().map(|(k, _)| k)
    }
}

pub struct RBTreeSetIntoIter<K>
where
    K: Ord,
{
    map_into_iter: RBTreeMapIntoIter<K, ()>,
}

impl<K> Iterator for RBTreeSetIntoIter<K>
where
    K: Ord,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_into_iter.next().map(|(k, _)| k)
    }
}

impl<K> DoubleEndedIterator for RBTreeSetIntoIter<K>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.map_into_iter.next_back().map(|(k, _)| k)
    }
}

impl<K> IntoIterator for RBTreeSet<K>
where
    K: Ord,
{
    type Item = K;
    type IntoIter = RBTreeSetIntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        RBTreeSetIntoIter {
            map_into_iter: self.map.into_iter(),
        }
    }
}

pub struct RBTreeSetUnion<'a, K>
where
    K: Ord,
{
    iters: [RBTreeSetIter<'a, K>; 2],
    head_vals: [Option<&'a K>; 2],
    tail_vals: [Option<&'a K>; 2],
}

impl<'a, K> RBTreeSetUnion<'a, K>
where
    K: Ord,
{
    fn new(iter_a: RBTreeSetIter<'a, K>, iter_b: RBTreeSetIter<'a, K>) -> Self {
        Self {
            iters: [iter_a, iter_b],
            head_vals: [None, None],
            tail_vals: [None, None],
        }
    }
}

impl<'a, K> Iterator for RBTreeSetUnion<'a, K>
where
    K: Ord,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..2 {
            if self.head_vals[i].is_none() {
                self.head_vals[i] = self.iters[i].next();
            }
        }
        match (self.head_vals[0], self.head_vals[1]) {
            (None, None) => None,
            (Some(v), None) => {
                self.head_vals[0] = None;
                Some(v)
            }
            (None, Some(v)) => {
                self.head_vals[1] = None;
                Some(v)
            }
            (Some(a), Some(b)) => {
                if a == b {
                    self.head_vals[0] = None;
                    self.head_vals[1] = None;
                    Some(a)
                } else if a < b {
                    self.head_vals[0] = None;
                    Some(a)
                } else {
                    self.head_vals[1] = None;
                    Some(b)
                }
            }
        }
    }
}

impl<'a, K> DoubleEndedIterator for RBTreeSetUnion<'a, K>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        for i in 0..2 {
            if self.tail_vals[i].is_none() {
                self.tail_vals[i] = self.iters[i].next_back();
            }
        }
        match (self.tail_vals[0], self.tail_vals[1]) {
            (None, None) => None,
            (Some(v), None) => {
                self.tail_vals[0] = None;
                Some(v)
            }
            (None, Some(v)) => {
                self.tail_vals[1] = None;
                Some(v)
            }
            (Some(a), Some(b)) => {
                if a == b {
                    self.tail_vals[0] = None;
                    self.tail_vals[1] = None;
                    Some(a)
                } else if a < b {
                    self.tail_vals[1] = None;
                    Some(b)
                } else {
                    self.tail_vals[0] = None;
                    Some(a)
                }
            }
        }
    }
}
