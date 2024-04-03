pub struct Solution;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort();

        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];

        while left < right {
            let mid = (left + right) / 2;
            if Solution::can_form_pairs(&nums, mid, p) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn can_form_pairs(nums: &Vec<i32>, mid: i32, p: i32) -> bool {
        let mut count = 0;
        let mut i = 0;
        while i < nums.len() - 1 && count < p {
            if nums[i + 1] - nums[i] <= mid {
                count += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        count >= p
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
        assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
        assert_eq!(Solution::minimize_max(vec![1, 2, 3, 4, 7], 2), 1);
    }
}
