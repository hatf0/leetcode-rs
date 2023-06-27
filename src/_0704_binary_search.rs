struct Solution;

impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut window = (0, nums.len());

    match nums.len() {
      1 => if nums[0] == target { 0 } else { -1 },
      2 => if nums[0] == target { 0 } else if nums[1] == target { 1 } else { -1 },
      _ => loop {
        let middle = ((window.1 - window.0) / 2) + window.0;
        if target < nums[middle] {
          window = (window.0, middle);
        } else if target > nums[middle] {
          window = (middle, window.1);
        } else {
          return middle as i32;
        }

        // if we've narrowed down our window and couldn't find a match,
        // return here
        if nums[window.0] == target {
          return window.0 as i32;
        }

        if window.1 - window.0 <= 1 {
          return -1;
        }
      }
    }
  }
}

#[test]
fn test() {
  assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
  assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
}