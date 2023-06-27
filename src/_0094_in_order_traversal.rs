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
use std::collections::VecDeque;


// XXX: THIS IS SO UNGODLY DISGUSTING
// PLEASE FOR THE LOVE OF GOD OPTIMIZE THIS CODE
// SO MANY UNNECESSARY CLONES
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      if root.is_none() {
        return Vec::new();
      }

      let tree_root = root.unwrap().clone();

      let mut curr = Some(tree_root.clone());
      let mut stack: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
      let mut result = Vec::new();
      // stack.push_back(tree_root);

      while curr.clone().is_some() || stack.len() != 0 {
        while curr.clone().is_some() {
          stack.push_front(curr.clone().unwrap());
          curr = curr.clone().unwrap().as_ref().borrow().left.clone();
        }

        curr = stack.pop_front();
        result.push(curr.clone().unwrap().as_ref().borrow().val);
        curr = curr.clone().unwrap().as_ref().borrow().right.clone();
      }

      result
    }
}

#[test]
fn test() {
  let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));

  if let Some(ref root) = root {
    let mut root = root.borrow_mut();
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    if let Some(left) = &root.left {
      left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3)))); 
      left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4)))); 
    }

    if let Some(right) = &root.right {
      right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5)))); 
      right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6)))); 
    }
  }

  Solution::inorder_traversal(Some(root.unwrap().clone()));

}