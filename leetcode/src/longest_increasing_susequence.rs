pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(Vec::new(), |mut acc: Vec<(i32, usize)>, elm: &i32| {
                let len = acc
                    .iter()
                    .filter(|v| v.0 < *elm)
                    .map(|v| v.1)
                    .max()
                    .unwrap_or(0);
                match acc.iter_mut().find(|v| v.1 == len + 1) {
                    Some(v) => {
                        if v.0 > *elm {
                            v.0 = *elm;
                        }
                    }
                    None => {
                        acc.push((*elm, len + 1));
                    }
                }
                acc
            })
            .iter()
            .map(|x| x.1)
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
