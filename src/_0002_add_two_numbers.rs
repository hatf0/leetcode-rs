use std::collections::VecDeque;

struct Solution;

// Definition for singly-linked list.
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

impl TryFrom<u64> for Box<ListNode> {
  type Error = ();

  fn try_from(v: u64) -> Result<Box<ListNode>, Self::Error> {
    let mut val = v;
    let mut head = Box::new(ListNode::new((val % 10) as i32));
    let mut item = Some(&mut head);
    val /= 10;

    while val != 0 {
      let it = item.unwrap();
      it.next = Some(Box::new(ListNode::new((val % 10) as i32)));
      val /= 10;
      item = it.next.as_mut();
    }

    Ok(head)
  }
}

impl TryFrom<Box<ListNode>> for u64 {
  type Error = ();

  fn try_from(v: Box<ListNode>) -> Result<u64, Self::Error> {
    let mut val: u64 = 0;
    let mut i = 0;
    let mut item = Some(&v);

    while let Some(ref node) = item {
      if i != 0 {
        val += (node.val as u64) * (10_u64.pow(i));
      }
      else {
        val += node.val as u64;
      }
      i += 1;
      item = node.next.as_ref();
    }

    Ok(val)
  }
}

impl TryFrom<Box<ListNode>> for VecDeque<u8> {
  type Error = ();

  fn try_from(v: Box<ListNode>) -> Result<VecDeque<u8>, Self::Error> {
    let mut val = VecDeque::default();
    let mut item = Some(&v);

    while let Some(ref node) = item {
      val.push_front(node.val as u8);
      item = node.next.as_ref();
    }

    Ok(val)
  }

}

impl TryFrom<VecDeque<u8>> for Box<ListNode> {
  type Error = ();

  fn try_from(v: VecDeque<u8>) -> Result<Box<ListNode>, Self::Error> {
    let mut val = v.iter().rev();
    let mut head = Box::new(ListNode::new(*val.nth(0).unwrap() as i32));
    let mut item = Some(&mut head);

    for v in val {
      let it = item.unwrap();
      it.next = Some(Box::new(ListNode::new(*v as i32)));
      item = it.next.as_mut();
    }

    Ok(head)
  }
}

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut lhs: VecDeque<u8> = l1.unwrap().try_into().unwrap();
    let mut rhs: VecDeque<u8> = l2.unwrap().try_into().unwrap();

    // make the two iterators equal in length
    if rhs.len() < lhs.len() {
      let diff = lhs.len() - rhs.len();
      for i in 0 .. diff {
        rhs.push_front(0);
      }
    } else if rhs.len() > lhs.len() {
      let diff = rhs.len() - lhs.len();
      for i in 0 .. diff {
        lhs.push_front(0);
      }
    }

    let it = lhs.iter_mut()
       .rev()
       .zip(rhs.iter().rev());

    let mut carry = 0;
    for (l, r) in it {
      let mut lpr = *l + r + carry;
      carry = 0;
      while lpr > 9 {
        carry += lpr / 10;
        lpr = lpr - 10;
      }
      println!("add {:?} and {:?}, result: {:?}, carry: {:?}", l, r, lpr, carry);
      *l = lpr;
    }

    if carry != 0 {
      lhs.push_front(carry);
    }

    lhs.try_into().ok()
  }
}

#[test]
fn test() {
  {
    let a: Option<Box<ListNode>> = 123.try_into().ok();
    let b: VecDeque<u8> = a.unwrap().try_into().unwrap();
    let c: Option<Box<ListNode>> = b.try_into().ok();
    assert_eq!(123.try_into().ok(), c);
    // start from the
  }
  // assert_eq!(Solution::add_two_numbers(123.try_into().ok(), 456.try_into().ok()), 579.try_into().ok());
  assert_eq!(Solution::add_two_numbers(9999999.try_into().ok(), 9999.try_into().ok()), 10009998.try_into().ok());
}