struct Solution;

use std::collections::VecDeque;
impl Solution {
  pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
    let original_color = image[sr as usize][sc as usize];
    if original_color == color {
      return image;
    }
    
    stack.push_back((sr, sc));

    while !stack.is_empty() {
      let (row, col) = stack.pop_front().unwrap();
      if row < 0 || row >= image.len() as i32 {
        continue;
      }

      if col < 0 || col >= image[row as usize].len() as i32 {
        continue; 
      }

      if image[row as usize][col as usize] != original_color {
        continue; 
      }

      image[row as usize][col as usize] = color;

      stack.push_back((row + 1, col));
      stack.push_back((row - 1, col));
      stack.push_back((row, col + 1));
      stack.push_back((row, col - 1));
    }

    image
  }
}

#[test]
fn test() {
  assert_eq!(Solution::flood_fill(vec![vec![1,1,1],vec![1,1,0],vec![1,0,1]], 1, 1, 2), vec![vec![2,2,2],vec![2,2,0],vec![2,0,1]]);
}