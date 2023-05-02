use std::collections::BTreeSet;
struct Solution;

impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut uniq_set: BTreeSet<i32> = BTreeSet::new();

    for num in nums.clone() {
      uniq_set.insert(num);
    }

    nums.clear();

    for num in uniq_set {
      nums.push(num);
    }

    nums.len() as i32
  }
}

#[test]
fn test() {
  {
    let mut arr = vec![0,0,1,1,1,2,2,3,3,4];
    assert_eq!(Solution::remove_duplicates(&mut arr), 5);
    assert_eq!(arr, vec![0,1,2,3,4]);
  }
}