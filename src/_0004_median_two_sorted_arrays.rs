struct Solution;

impl Solution {
  pub fn find_median_sorted_arrays(n1: Vec<i32>, n2: Vec<i32>) -> f64 {
    let mut nums = n1.clone();
    nums.append(&mut n2.clone());
    nums.sort();

    let median_ind = nums.len() / 2; 
    match nums.len() % 2 {
      0 => {
        (nums[median_ind - 1] as f64 + nums[median_ind] as f64) / 2.0
      }
      1 => {
        nums[median_ind] as f64
      }
      _ => panic!("unexpected result from modulus")
    }
  }
}

#[test]
fn test() {
  assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
  assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
}
