struct Solution;
use itertools::{iproduct, Itertools};

impl Solution {
    // Given an integer array nums, return all the triplets
    // [nums[i], nums[j], nums[k]] such that
    // i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
    //
    // Notice that the solution set must not contain duplicate triplets.
    // O(n^3) space complexity, but who's counting?
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let arr = iproduct!(
            nums.iter().enumerate(),
            nums.iter().enumerate(),
            nums.iter().enumerate()
        );

        arr.filter(|((i, _), (j, _), (k, _))| i != j && i != k && j != k)
            .filter(|((_, x), (_, y), (_, z))| **x + **y + **z == 0)
            .unique_by(|((i, _), (j, _), (k, _))| i + j + k)
            .map(|((_, x), (_, y), (_, z))| vec![*x, *y, *z])
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, 0, 1], vec![-1, 2, -1]]
    );
    let empty: Vec<Vec<i32>> = Vec::default();
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), empty);
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
}
