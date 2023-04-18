struct Solution;

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut tmp = x;
    if x < 0 {
      tmp *= -1;
    }

    let num_digits = ((tmp as f32).log10() as i32) + 1;
    let mut out: u32 = 0;
    
    for i in 0 .. num_digits {
      let digit = tmp % 10;
      tmp = tmp / 10;
      out += digit as u32;
      if i < num_digits - 1 {
        if let Some(result) = out.checked_mul(10) {
          out = result;
        } else {
          return 0;
        }
      }
    }

    if let Ok(result) = i32::try_from(out) {
      if x < 0 {
        result * -1
      }
      else {
        result
      }
    } else {
      0
    }
  }
}

#[test]
fn test() {
  assert_eq!(Solution::reverse(123), 321);
  assert_eq!(Solution::reverse(-123), -321);
  assert_eq!(Solution::reverse(1534236469), 0);
}