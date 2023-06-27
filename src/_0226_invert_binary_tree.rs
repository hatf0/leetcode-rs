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
      right: None
    }
  }
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      if root.is_none() {
        return root
      }

      let root = root.unwrap();

      let root_clone = Rc::clone(&root);
      let root_ref = RefCell::borrow(&root_clone);
      if let Some(left) = &root_ref.left {
          Solution::invert_tree(Some(Rc::clone(left)));
      }
      if let Some(right) = &root_ref.right {
          Solution::invert_tree(Some(Rc::clone(right)));
      }

      let tmp = root_ref.left.clone();
      let right = root_ref.right.clone();
      std::mem::drop(root_ref);
      
      root.borrow_mut().left = right;
      root.borrow_mut().right = tmp;

      Some(root)
  }
}