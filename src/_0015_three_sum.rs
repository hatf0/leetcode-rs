struct Solution;
use std::collections::BTreeSet;

impl Solution {
    // Given an integer array nums, return all the triplets
    // [nums[i], nums[j], nums[k]] such that
    // i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
    //
    // Notice that the solution set must not contain duplicate triplets.
    // O(n^3) space complexity, but who's counting?
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let sums: Vec<((usize, usize), i32)> = nums.iter().clone().enumerate().flat_map(|(i, x)| { 
            nums.iter().clone().enumerate().map(move |(j, y)| ((i, j), x + y))
        })
        .filter(|((i, j), _)| i != j)
        .collect();

        let mut iter = sums.iter();      
        let mut out: Vec<Vec<i32>> = Vec::new();  
        let mut index_sums_set : BTreeSet<i32> = BTreeSet::new();
        let mut sums_set: BTreeSet<[i32; 3]> = BTreeSet::new();

        while let Some(((i, j), result)) = iter.next() {
            // we want to find a pair that'll bring us back to zero
            let target = *result * -1;
            // println!("need to find {:?} to fix {:?} + {:?}", target, nums[*i], nums[*j]);
            let result = nums.iter()
                                 .enumerate()
                                 .filter(|(k, _)| i != k && j != k)
                                 .find(|(_, x)| **x == target);

            if let Some((k, _)) = result {
                let triplet = [nums[*i], nums[*j], nums[k]];
                if !sums_set.contains(&[0, 0, 0]) && triplet == [0, 0, 0] {
                    out.push(vec![0, 0, 0]);
                    sums_set.insert([0, 0, 0]);
                    continue;
                }

                println!("potential: {:?} + {:?} + {:?}", nums[*i], nums[*j], nums[k]);

                if sums_set.iter().find(|x| {
                    let mut set: BTreeSet<i32> = BTreeSet::new();
                    println!("{:?}, {:?}", **x, triplet);
                    if **x == triplet {
                        return true;
                    }

                    if **x == [0, 0, 0] {
                        return false;
                    }

                    for item in **x {
                        set.insert(item);
                    }
                    for item in triplet {
                        set.insert(item);
                    }

                    set.len() == 3
                }).is_some() {
                    continue;
                }

                println!("found: {:?} + {:?} + {:?}", nums[*i], nums[*j], nums[k]);
                // println!("{:?} {:?} {:?}", *i, *j, k);
                out.push(vec![nums[*i], nums[*j], nums[k]]);
                sums_set.insert(triplet);
            }
        }

        out
    }
}

#[test]
fn test() {
    // assert_eq!(
    //     Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
    //     vec![vec![-1, 0, 1], vec![-1, 2, -1]]
    // );
    let empty: Vec<Vec<i32>> = Vec::default();
    // assert_eq!(Solution::three_sum(vec![0, 1, 1]), empty);
    // assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    // assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    // assert_eq!(Solution::three_sum(vec![-1,0,1,0]), vec![vec![-1, 0, 1]]);
    assert_eq!(Solution::three_sum(vec![-4,-2,1,-5,-4,-4,4,-2,0,4,0,-2,3,1,-5,0]), vec![vec![-5,1,4],vec![-4,0,4],vec![-4,1,3],vec![-2,-2,4],vec![-2,1,1],vec![0,0,0]]);
}
