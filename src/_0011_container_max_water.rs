struct Solution;

type Point = (i32, i32);

impl Solution {
  pub fn get_volume(lhs: Point, rhs: Point) -> i32 {
    let min_height = lhs.1.min(rhs.1);
    (lhs.0 - rhs.0).abs() * min_height
  }

  pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_vol = 0;
    let mut lhs = 0;
    let mut rhs = height.len() - 1;

    loop {
      if lhs >= rhs {
        break;
      }

      let vol = Solution::get_volume((lhs as i32, height[lhs]), (rhs as i32, height[rhs]));
      max_vol = max_vol.max(vol);

      if height[lhs] > height[rhs] {
        rhs -= 1
      } else {
        lhs += 1
      }
    }

    max_vol
  }

}

#[test]
fn test() {
  assert_eq!(Solution::get_volume((1, 8), (8, 7)), 49);
  assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
  assert_eq!(Solution::max_area(vec![1,1]), 1);
}