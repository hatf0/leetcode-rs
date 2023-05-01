struct Solution;

/*
You are given a string s and an integer array indices of the same length. The string s will be shuffled such that the character at the ith position moves to indices[i] in the shuffled string.

Return the shuffled string.
 */
impl Solution {
  pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let s_vec = s.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    for _ in 0 .. s.len() {
      out.push(0);
    }

    for i in 0 .. indices.len() {
      out[indices[i] as usize] = s_vec[i];
    }

    String::from_utf8(out).unwrap()
  }
}

#[test]
fn test() {
  assert_eq!(Solution::restore_string("codeleet".to_string(), vec![4,5,6,7,0,2,1,3]), "leetcode".to_string());
  assert_eq!(Solution::restore_string("abc".to_string(), vec![0,1,2]), "abc".to_string());
}