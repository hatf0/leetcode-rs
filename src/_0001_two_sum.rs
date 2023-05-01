use std::collections::BTreeMap;

struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_map: BTreeMap<i32, usize> = BTreeMap::new();

    for i in 0 .. nums.len() {
      let diff = target - nums[i];

      if let Some(diff_ind) = sum_map.get(&diff) {
        return vec![*diff_ind as i32, i as i32]
      }

      sum_map.insert(nums[i], i);
    }

    panic!("should never happen")
  }
}

#[test]
fn test() {
  assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1]);
  assert_eq!(Solution::two_sum(vec![3,2,95,4,-3], 92), vec![2, 4]);
  assert_eq!(Solution::two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
}