struct Solution;

impl Solution {
  pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut result = 1.0;
    let mut power = n;
    let mut a = x;

    if power == 0 {
      return 1.0;
    }

    if power < 0 {
      power = power.abs();
    }

    if n == i32::MIN {
      return if x == 1.0 {
        1.0
      } else {
        if x == -1.0 {
          1.0
        } else {
          0.0
        }
      }
    }

    if n == i32::MAX {
      return if x.abs() == 1.0 {
        1.0
      } else {
        0.0
      }
    }

    while power > 0 {
      if power & 1 == 1 {
        result *= a;
      }

      a *= a;
      power >>= 1;
    }

    if n < 0 {
      1.0 / result
    } else {
      result
    }
  }
}

#[test]
fn test() {
  assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
  assert_eq!(Solution::my_pow(2.0, -2), 0.25);
  assert_eq!(Solution::my_pow(2.1, 3), 9.2610);

}