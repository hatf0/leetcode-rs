struct Solution;

impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = i32::MIN;
    let mut min_price = i32::MAX;
    for i in 0 .. prices.len() {
      let price = prices[i];

      if price < min_price {
        min_price = price
      }

      if price - min_price > max_profit {
        max_profit = price - min_price
      }
    }

    max_profit
  }
}

#[test]
fn test() {
  assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
}