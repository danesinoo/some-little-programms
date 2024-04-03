use std::collections::BTreeMap;
pub struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|((start, end), profit)| (start, end, profit))
            .collect::<Vec<(i32, i32, i32)>>();

        jobs.sort_by_key(|(_, e, _)| *e);
        let mut schedule = BTreeMap::new();
        schedule.insert(0, 0);

        for (start, end, profit) in jobs {
            let max = match schedule.range(..&start + 1).rev().next() {
                Some(profit) => *profit.1,
                None => unreachable!(),
            };

            match schedule.range_mut(..&end + 1).rev().next() {
                Some((end_time, prev_profit)) => {
                    if *prev_profit < max + profit {
                        if *end_time < end {
                            schedule.insert(end, max + profit);
                        } else {
                            *prev_profit = max + profit;
                        }
                    }
                }
                None => unreachable!(),
            }
        }
        *schedule.last_key_value().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_scheduling() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );
        assert_eq!(
            Solution::job_scheduling(
                vec![6, 15, 7, 11, 1, 3, 16, 2],
                vec![19, 18, 19, 16, 10, 8, 19, 8],
                vec![2, 9, 1, 19, 5, 7, 3, 19]
            ),
            41
        );
    }
}
