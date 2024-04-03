use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut dp = vec![HashMap::new(); nums.len()];

        for i in 0..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let count = *dp[j].get(&diff).unwrap_or(&0);
                *dp[i].entry(diff).or_insert(0) += count + 1;
                total += count;
            }
        }

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_arithmetic_slices() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
            0
        );
    }
}
