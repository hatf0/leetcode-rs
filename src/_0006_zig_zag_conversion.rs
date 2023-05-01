use core::num;

struct Solution;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
      return s;
    }

    let mut rows: Vec<String> = Vec::default();
    for _ in 0 .. num_rows {
      rows.push("".to_string());
    }
    let mut ind = 0;
    let mut dir = -1;
    let chars = s.as_bytes();

    for i in 0 .. s.len() {
      if ind == num_rows - 1 || ind == 0 {
        dir *= -1;
      }

      if let Some(row) = rows.get_mut(ind as usize) {
        row.push(chars[i] as char);
      }

      ind += dir;
    }


    let mut out = String::default();
    for row in rows {
      out.push_str(&row);
    }

    out
  }
}

#[test]
fn test() {
  // assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
  assert_eq!(Solution::convert("AB".to_string(), 1), "AB".to_string());
}