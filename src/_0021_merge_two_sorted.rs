struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

use std::convert::{TryFrom, TryInto};

impl TryFrom<Box<ListNode>> for Vec<i32> {
  type Error = ();

  fn try_from(value: Box<ListNode>) -> Result<Self, Self::Error> {
    let mut out = Vec::new();
    let mut item = Some(&value);
    while item.is_some() {
      out.push(item.unwrap().val);
      item = item.unwrap().next.as_ref();
    }

    Ok(out)
  }
}

impl TryFrom<Vec<i32>> for Box<ListNode> {
  type Error = ();

  fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
    if value.len() < 1 {
      return Err(());
    }

    let mut iter = value.iter();

    let mut head = Box::new(ListNode {
      next: None,
      val: iter.nth(0).unwrap().clone()
    });

    let mut item = head.as_mut();

    while let Some(val) = iter.nth(0) {
      let next = Box::new(ListNode {
        next: None,
        val: val.clone()
      });

      item.next = Some(next);
      item = item.next.as_mut().unwrap().as_mut();
    }

    Ok(head)
  }  
}

impl Solution {
  pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if list1.is_none() {
      return list2;
    }

    if list2.is_none() {
      return list1;
    }

    let l1: Vec<i32> = list1.unwrap().try_into().unwrap();
    let l2: Vec<i32> = list2.unwrap().try_into().unwrap();

    let mut merged = [l1, l2].concat();
    merged.sort();
      
    merged.try_into().ok()            
  }
}