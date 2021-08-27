use std::rc::Rc;
use std::cell::RefCell;

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
			right: None
		}
	}
}

fn main() {
	let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
	// let node = &root.clone().unwrap();
	// let _left = RefCell::borrow(node).left.clone();

	// error
	let node = root.as_ref().unwrap();
	let _right = RefCell::borrow(node).right.clone();
}