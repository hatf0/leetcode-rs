use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};

struct Nums(VecDeque<u8>);

impl TryFrom<i32> for Nums {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    let mut work = value;
    let mut out = VecDeque::new();

    while work > 0 {
      let val = work % 10;
      work /= 10;
      out.push_front(val as u8);
    }

    Ok(Nums(out))
  }
}

struct Solution;

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
      if x < 0 {
        return false;
      }

      let digits: Nums = x.try_into().unwrap();

      let mut iter = digits.0.iter()
              .enumerate()
              .zip(digits.0.iter().enumerate().rev());

      while let Some(item) = iter.next() {
        if item.0.0 >= item.1.0 {
          break;
        }

        if item.0.1 != item.1.1 {
          return false;
        }
      }

      true
  }
}

#[test]
fn test() {
  assert!(Solution::is_palindrome(1221));
  assert!(Solution::is_palindrome(121));
  assert_eq!(Solution::is_palindrome(1234), false);
}