use std::collections::BTreeMap;

struct Solution;

impl Solution {
  pub fn is_palindrome(s: String) -> bool {
    if s.len() == 1 {
      return true;
    }

    let mut iter = s.as_bytes()
                .iter()
                .enumerate()
                .zip(s.as_bytes().iter().enumerate().rev());

    while let Some(((i, x), (j, y))) = iter.next() {
      if i >= j {
        break;
      }

      if x != y {
        return false;
      }
    }

    true
  }
  pub fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
      return s;
    }

    if s.len() == 0 {
      return "".to_string();
    }

    if Solution::is_palindrome(s.clone()) {
      return s;
    }

    let mut palindrome_map: BTreeMap<&str, bool> = BTreeMap::new();
    
    // generate our map of palindromes here ... this is going to be a little yucky!
    let it = 0..s.len();
    
    let mut product = it.clone().flat_map(|x| it.clone().map(move |y| (x, y)))
                                                                  .filter(|((i, j))| i <= j);

    for i in 0 .. s.len() {
      let slice = &s[0 ..= i];
      if palindrome_map.contains_key(slice) {
        continue;
      }

      println!("{slice}");

      palindrome_map.insert(slice, Solution::is_palindrome(slice.to_string()));
    }

    while let Some((start, end)) = product.next() {
      let slice = &s[start ..= end];
      if palindrome_map.contains_key(slice) {
        continue;
      }

      // println!("{start}, {end}, {slice}");

      palindrome_map.insert(slice, Solution::is_palindrome(slice.to_string()));
    }

    let max = palindrome_map.iter()
                  .filter(|(_, is_palindrome)| **is_palindrome)
                  .max_by(|(x, _), (y, _)| x.len().cmp(&y.len()))
                  .unwrap_or((&"", &false));

    max.0.to_string()
  }
}

#[test]
fn test() {
  // assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
  // assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
  // assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb".to_string());
  // assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb".to_string());
  // assert_eq!(Solution::longest_palindrome("abb".to_string()), "bb".to_string());
  // assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
  assert_eq!(Solution::longest_palindrome("whdqcudjpisufnrtsyupwtnnbsvfptrcgvobbjglmpynebblpigaflpbezjvjgbmofejyjssdhbgghgrhzuplbeptpaecfdanhlylgusptlgobkqnulxvnwuzwauewcplnvcwowmbxxnhsdmgxtvbfgnuqdpxennqglgmspbagvmjcmzmbsuacxlqfxjggrwsnbblnnwisvmpwwhomyjylbtedzrptejjsaiqzprnadkjxeqfdpkddmbzokkegtypxaafodjdwirynzurzkjzrkufsokhcdkajwmqvhcbzcnysrbsfxhfvtodqabvbuosxtonbpmgoemcgkudandrioncjigbyizekiakmrfjvezuzddjxqyevyenuebfwugqelxwpirsoyixowcmtgosuggrkdciehktojageynqkazsqxraimeopcsjxcdtzhlbvtlvzytgblwkmbfwmggrkpioeofkrmfdgfwknrbaimhefpzckrzwdvddhdqujffwvtvfyjlimkljrsnnhudyejcrtrwvtsbkxaplchgbikscfcbhovlepdojmqybzhbiionyjxqsmquehkhzdiawfxunguhqhkxqdiiwsbuhosebxrpcstpklukjcsnnzpbylzaoyrmyjatuovmaqiwfdfwyhugbeehdzeozdrvcvghekusiahfxhlzclhbegdnvkzeoafodnqbtanfwixjzirnoaiqamjgkcapeopbzbgtxsjhqurbpbuduqjziznblrhxbydxsmtjdfeepntijqpkuwmqezkhnkwbvwgnkxmkyhlbfuwaslmjzlhocsgtoujabbexvxweigplmlewumcone".to_string()), "".to_string());
}