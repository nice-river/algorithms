#![allow(clippy::items_after_test_module)]
#![allow(dead_code, unused_imports, unused_variables)]
mod lcc488q4;

use std::cell::RefCell;
use std::rc::Rc;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(unused_macros)]
macro_rules! to_vec {
    ([ [$($x:tt),* $(,)?] $(,)?] $(,)?) => {
        // e.g. [[ [1], [2, 3], [4] ]]
        // e.g. [[ 1, 2, 3, ]]
        vec![vec![ $(to_vec!($x)),* ]]
    };
    ([ [$($x:expr),* $(,)?] $(,)?] $(,)?) => {
        // e.g. [[ 1 + 2, 3 * 4 ]]
        // 1 + 2 is not a valid `tt`
        vec![vec![ $(to_vec!($x)),* ]]
    };
    ([ [$($x:tt),* $(,)?], $($y:tt),+ $(,)?] $(,)?) => {
        // e.g. [[ [1], [2, 3], [4] ], [[5, 6], []]]
        // e.g. [[ 1, 2, 3, ], []]
        {
            let mut x = vec![vec![$(to_vec!($x)),* ]];
            x.extend([$(to_vec!($y)),+]);
            x
        }
    };
    ([ [$($x:expr),* $(,)?], $($y:tt),+ $(,)?] $(,)?) => {
        // e.g. [[ 1 + 2, 2 + 3, 4, ], [], [2, 3]]
        // 1 + 2 is not a valid `tt`
        {
            let mut x = vec![vec![$(to_vec!($x)),* ]];
            x.extend([$(to_vec!($y)),+]);
            x
        }
    };
    ([ $($x:expr),* $(,)?] $(,)?) => {
        // [ 1 + 2, 2 + 3, 4, ],
        vec![ $(to_vec!($x)),* ]
    };
    ($x:expr) => {
        $x
    }
}

pub(crate) use to_vec;

fn test_macro_to_vec() {
    let x = 20;
    let y = 30;
    let s = to_vec!([
        [[1, 2 + 3,], [x * y], [x]],
        [[if x > y { x } else { y }]],
        [[1, 2 + 3,], [x * y,], [x], []],
        [[], [y, 2], [x * 10],],
        [
            [y + 2],
            [y, 2],
            [x],
            [match x > y {
                true => x - y,
                false => x + y,
            }]
        ],
    ],);
    let t = vec![
        vec![vec![1, 2 + 3], vec![x * y], vec![x]],
        vec![vec![if x > y { x } else { y }]],
        vec![vec![1, 2 + 3], vec![x * y], vec![x], vec![]],
        vec![vec![], vec![y, 2], vec![x * 10]],
        vec![
            vec![y + 2],
            vec![y, 2],
            vec![x],
            vec![match x > y {
                true => x - y,
                false => x + y,
            }],
        ],
    ];
    assert_eq!(s, t);
}
