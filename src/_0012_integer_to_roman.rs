struct Solution;

impl Solution {
  pub fn int_to_roman(num: i32) -> String {
    let mut out = String::default();
    let mut val = num;

    // for the most part, roman numbers are represented
    // with their numerals (I, V, X, L, C, D, M) EXCEPT
    // in the cases of 4 and 9 (in any 10s place)

    // start from the very top:
    for i in 0 .. (num / (10_i32.pow(3))) {
      out.push('M');
    }

    val = val % (10_i32.pow(3));

    match val / (10_i32.pow(2)) {
      i @ 0 ..= 3 => {
        for _ in 0 .. i {
          out.push('C');
        }
      },
      4 => out.push_str("CD"),
      9 => out.push_str("CM"),
      i @ 5 ..= 8 => {
        out.push('D');
        for _ in 0 .. (i - 5) {
          out.push('C');
        }
      },
      _ => unreachable!("unexpected"),
    }

    val = val % (10_i32.pow(2));

    match val / (10_i32.pow(1)) {
      i @ 0 ..= 3 => {
        for _ in 0 .. i {
          out.push('X');
        }
      },
      4 => out.push_str("XL"),
      9 => out.push_str("XC"),
      i @ 5 ..= 8 => {
        out.push('L');
        for _ in 0 .. (i - 5) {
          out.push('X');
        }
      },
      _ => unreachable!("unexpected"),
    }

    val = val % 10;

    match val {
      i @ 0 ..= 3 => {
        for _ in 0 .. i {
          out.push('I');
        }
      },
      4 => out.push_str("IV"),
      9 => out.push_str("IX"),
      i @ 5 ..= 8 => {
        out.push('V');
        for _ in 0 .. (i - 5) {
          out.push('I');
        }
      },
      _ => unreachable!("unexpected"),
    }

    out
  }
}

#[test]
fn test() {
  assert_eq!(Solution::int_to_roman(3), "III");
  assert_eq!(Solution::int_to_roman(58), "LVIII");
  assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}