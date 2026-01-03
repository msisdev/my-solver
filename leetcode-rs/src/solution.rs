use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h: HashMap<i32, usize> = HashMap::new();

        for (i, &a) in nums.iter().enumerate() {
            let b = target- a;
            if let Some(&j) = h.get(&b) {
                return vec![i as i32, j as i32];
            }

            h.insert(a, i);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: Vec<i32>, target: i32, exp: Vec<i32>) {
        let mut act = Solution::two_sum(nums, target);
        act.sort();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_case_1() {
        test(
            vec![2, 7, 11, 15],
            9,
            vec![0, 1],
        );
    }

    #[test]
    fn test_case_2() {
        test(
            vec![3, 2, 4],
            6,
            vec![1, 2],
        );
    }

    #[test]
    fn test_case_3() {
        test(
            vec![3, 3],
            6,
            vec![0, 1],
        );
    }
}
