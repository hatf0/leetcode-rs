struct Solution;

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // the strategy is going to be iterate over every single string in parallel,
    // and take n characters while every character equals each other
    let mut out = String::default();
    for i in 0 .. strs.iter().map(|x| x.len()).min().unwrap() {
      let c = strs[0].as_bytes()[i];
      if !strs.iter().all(|x| x.as_bytes()[i] == c) {
        break;
      }

      out.push(c as char);
    }
    out
  }
}

#[test]
fn test() {
  assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl".to_string())
}