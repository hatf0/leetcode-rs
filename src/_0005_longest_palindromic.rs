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
    if s.len() <= 1 {
      return s;
    }

    if Solution::is_palindrome(s.clone()) {
      return s;
    }

    let mut dp: Vec<Vec<u8>> = Vec::new();
    for i in 0 .. s.len() {
      let mut row: Vec<u8> = Vec::new();
      for _ in 0 ..= i  {
        row.push(0);
      }

      dp.push(row);
    }

    let chars = s.as_bytes();

    // base case: P(i, i) == true
    // every one-letter string is a palindrome!
    for i in 0 .. s.len() {
      dp[i][i] = 1;

      // handle palindromes of len 2
      if i < s.len() - 1 {
        dp[i + 1][i] = (chars[i] == chars[i + 1]) as u8;
      }
    }

    let mut max_size = 0;
    let mut max_palindrome = "";

    for i in 0 .. s.len() {
      for j in (0 .. dp[i].len()).rev() {
        // println!("{i} {j}, {:?}, {:?}", &s[j ..= i], dp[i][j]);
        if i != 0 && j < dp[i - 1].len() - 1 {
          // println!("adj: {:?}", dp[i - 1][j + 1]);
          dp[i][j] = (dp[i - 1][j + 1] == 1 && chars[i] == chars[j]) as u8;
        }

        if dp[i][j] == 1 {
          let sz = (i + 1) - j;

          if sz > max_size {
            max_size = sz;
            max_palindrome = &s[j ..= i];
            // println!("new palindrome: {:?}, {:?}", max_size, max_palindrome);
          }
        }
      }
    }

    max_palindrome.to_string()
  }
}

#[test]
fn test() {
  assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
  assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
  assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb".to_string());
  assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb".to_string());
  assert_eq!(Solution::longest_palindrome("abb".to_string()), "bb".to_string());
  assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
  assert_eq!(Solution::longest_palindrome("whdqcudjpisufnrtsyupwtnnbsvfptrcgvobbjglmpynebblpigaflpbezjvjgbmofejyjssdhbgghgrhzuplbeptpaecfdanhlylgusptlgobkqnulxvnwuzwauewcplnvcwowmbxxnhsdmgxtvbfgnuqdpxennqglgmspbagvmjcmzmbsuacxlqfxjggrwsnbblnnwisvmpwwhomyjylbtedzrptejjsaiqzprnadkjxeqfdpkddmbzokkegtypxaafodjdwirynzurzkjzrkufsokhcdkajwmqvhcbzcnysrbsfxhfvtodqabvbuosxtonbpmgoemcgkudandrioncjigbyizekiakmrfjvezuzddjxqyevyenuebfwugqelxwpirsoyixowcmtgosuggrkdciehktojageynqkazsqxraimeopcsjxcdtzhlbvtlvzytgblwkmbfwmggrkpioeofkrmfdgfwknrbaimhefpzckrzwdvddhdqujffwvtvfyjlimkljrsnnhudyejcrtrwvtsbkxaplchgbikscfcbhovlepdojmqybzhbiionyjxqsmquehkhzdiawfxunguhqhkxqdiiwsbuhosebxrpcstpklukjcsnnzpbylzaoyrmyjatuovmaqiwfdfwyhugbeehdzeozdrvcvghekusiahfxhlzclhbegdnvkzeoafodnqbtanfwixjzirnoaiqamjgkcapeopbzbgtxsjhqurbpbuduqjziznblrhxbydxsmtjdfeepntijqpkuwmqezkhnkwbvwgnkxmkyhlbfuwaslmjzlhocsgtoujabbexvxweigplmlewumcone".to_string()), "".to_string());
}