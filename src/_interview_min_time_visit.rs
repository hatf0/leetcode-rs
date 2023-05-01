use std::convert::{TryInto, TryFrom};

struct Solution;

type Point = (i32, i32);

struct PointVec(Vec<i32>);

impl TryFrom<PointVec> for Point {
  type Error = ();

  fn try_from(value: PointVec) -> Result<Self, Self::Error> {
    if value.0.len() != 2 {
      Err(())
    } else {
      Ok((value.0[0], value.0[1]))
    }
  }
}

impl Solution {
  fn calculate_distance(start: Point, end: Point) -> f32 {
    // good ol x2 - x1 / y2 - y1
    (((end.0 - start.0) as f32).powf(2.0) + ((end.1 - start.1) as f32).powf(2.0)).sqrt()
  }

  fn vec_to_point(p: Vec<i32>) -> Point {
    PointVec(p).try_into().unwrap()
  }

  pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;
    let mut iter = points.windows(2);
    while let Some([svec, evec]) = iter.next() {
      let start = Solution::vec_to_point(svec.clone());
      let end = Solution::vec_to_point(evec.clone());
      let mut current = start;

      // println!("start: {:?}, end: {:?}", start, end);

      while current != end {
        // generate a matrix of distances
        let mut walk_matrix: Vec<(Point, f32)> = Vec::new();
        for x in -1 ..= 1 {
          for y in -1 ..= 1 {
            let new_point: Point = (current.0 + x, current.1 + y);
            walk_matrix.push((new_point, Solution::calculate_distance(new_point, end)));
          }
        }

        // println!("{:?}", walk_matrix);

        let min_dist = walk_matrix.iter()
                      .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap();
        
        // println!("walking to {:?}", min_dist.0);
        current = min_dist.0;
        result += 1;
      }
    }

    result
  }
}

#[test]
fn test() {
  assert_eq!(Solution::min_time_to_visit_all_points(vec![vec![1,1],vec![3,4],vec![-1,0]]), 7);
}