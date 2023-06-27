struct Solution;

use std::collections::HashMap;
impl Solution {
  fn frequency_map(s: String) -> HashMap<char, u32> {
    let mut ret = HashMap::new();

    for c in s.chars() {
      if ret.contains_key(&c) {
        ret.entry(c).and_modify(|e| { *e += 1 });
      } else {
        ret.insert(c, 1);
      }
    }

    ret
  }
  pub fn is_anagram(s: String, t: String) -> bool {
    let lhs = Solution::frequency_map(s);
    let rhs = Solution::frequency_map(t);

    lhs == rhs
  }
}