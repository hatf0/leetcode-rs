use std::collections::{BTreeMap,BTreeSet};

struct Solution;

impl Solution {
  pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();

    arr.iter()
       .for_each(|x| {
        map.entry(*x).and_modify(|x| *x += 1).or_insert_with(|| 1);
       });

    map.values()
       .collect::<BTreeSet<&i32>>()
       .len() == map.values().count()
  }
}

#[test]
fn test() {
  assert_eq!(Solution::unique_occurrences(vec![1,2,2,1,1,3]), true);
  assert_eq!(Solution::unique_occurrences(vec![1,2]), false);
}