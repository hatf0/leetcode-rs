struct Solution;

impl Solution {
  pub fn map_digit(digit: char) -> Vec<char> {
    match digit {
      '2' => vec!['a', 'b', 'c'],
      '3' => vec!['d', 'e', 'f'],
      '4' => vec!['g', 'h', 'i'],
      '5' => vec!['j', 'k', 'l'],
      '6' => vec!['m', 'n', 'o'],
      '7' => vec!['p', 'q', 'r', 's'],
      '8' => vec!['t', 'u', 'v'],
      '9' => vec!['w', 'x', 'y', 'z'],
      _ => panic!("unexpected"),
    }
  }

  pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 1 {
      return Solution::map_digit(digits.chars().nth(0).unwrap()).iter().map(|x| x.to_string()).collect();
    }
    let iter = digits.chars()
                             .enumerate()
                             .map(|x| (x.0, Solution::map_digit(x.1)));
    let permutations = iter.clone()
                           .flat_map(move |x| iter.clone().filter(move |y| x.0 != y.0 && x.0 < y.0).map(move |y| (x.1.clone(), y.1)))
                           .flat_map(|(x, y)| {
                            let mut results: Vec<String> = Vec::new();
                            for i in 0 .. x.len() {
                              for j in 0 .. y.len() {
                                results.push(format!("{}{}", x[i], y[j]));
                              }
                            }
                            results
                           })
                           .collect();

    permutations
  }
}

#[test]
fn test() {
  assert_eq!(Solution::letter_combinations("23".to_string()), vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
  assert_eq!(Solution::letter_combinations("".to_string()), Vec::new() as Vec<String>);
  assert_eq!(Solution::letter_combinations("2".to_string()), vec!["a", "b", "c"])
}