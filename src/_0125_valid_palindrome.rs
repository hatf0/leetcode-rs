struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let lower = s.to_ascii_lowercase();
        let input = lower.trim();

        if s.len() == 1 {
            return true;
        }

        let alnum = input.as_bytes().iter().filter(|x| x.is_ascii_alphanumeric()).collect::<Vec<&u8>>();
        let base_iter = alnum.iter();
        let mut iter = base_iter.clone()
            .enumerate()
            .zip(base_iter.enumerate().rev());

        while let Some(((i, x), (j, y))) = iter.next() {
            if i > j {
                break;
            }

            if x != y {
                return false;
            }
        }

        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
}
