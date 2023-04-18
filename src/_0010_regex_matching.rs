struct Solution;

impl Solution {
    pub fn is_match(input: String, pattern: String) -> bool {
        // we don't need to implement any form of backtracking or anything here,
        // but we do need to maintain a running state of whether or not the pattern matches

        let mut match_state: Vec<bool> =
            Vec::with_capacity((input.len() + 1) * (pattern.len() + 1));
        for _ in 0..match_state.capacity() {
            match_state.push(false);
        }

        let get_match_state = |match_state: &Vec<bool>, row: usize, col: usize| {
            match_state[row * (input.len() + 1) + col]
        };

        let set_match_state =
            |match_state: &mut Vec<bool>, row: usize, col: usize, val: bool| {
                match_state[row * (input.len() + 1) + col] = val;
            };

        // // since we'll be cascading forward
        // // make sure that the initial match state is true
        // set_match_state(&mut match_state, 0, 0, true);

        // for i in 2..=pattern.len() {
        //     let pattern_chars = pattern.as_str();
        //     // if this is a wildcard,
        //     // and the previous entry was a match,
        //     // then we should cascade this forward
        //     if let Some(pat) = pattern_chars.get(i - 1..i) {
        //         set_match_state(
        //             &mut match_state,
        //             0,
        //             i,
        //             pat == "*" && get_match_state(&match_state,x 0, i - 2),
        //         );
        //     }
        // }

        // for i in 1..=pattern.len() {
        //     for j in 1..=input.len() {
        //         let pattern_chars = pattern.as_str();
        //         let input_chars = input.as_str();

        //         // if the pattern matches the string exactly,
        //         // cascade the previous result forward
        //         if pattern_chars.get(i - 1..i) == input_chars.get(j - 1..j)
        //             || pattern_chars.get(i - 1..i).unwrap_or_default() == "."
        //         {
        //             set_match_state(&mut match_state, j, i, get_match_state(&match_state, j - 1, i - 1));
        //             continue;
        //         }

        //         if pattern_chars.get(i - 1..i).unwrap_or_default() == "*" {}
        //     }
        // }

        match_state[input.len() * pattern.len()]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_match("ab".to_owned(), ".*".to_owned()), true);
    assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
    assert_eq!(Solution::is_match("aa".to_owned(), "a*".to_owned()), true);
}
