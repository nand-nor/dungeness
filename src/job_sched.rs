use std::boxed::Box;
use std::cmp::max;

/* LC problem: maximum profit in job scheduling
* https://leetcode.com/problems/maximum-profit-in-job-scheduling/
*/

pub struct JobSched<T: FnMut(Vec<i32>, Vec<i32>, Vec<i32>) -> i32> {
    _fn_ptr: T,
}

use crate::problem::Solution;

impl<T: FnMut(Vec<i32>, Vec<i32>, Vec<i32>) -> i32> Solution for JobSched<T> {
    type ProblemFunc = T;
    type ProblemArgs = (Vec<i32>, Vec<i32>, Vec<i32>);
    type ProblemSolution = i32;
    fn solution(
        mut problem: Box<Self::ProblemFunc>,
        args: Self::ProblemArgs,
    ) -> Self::ProblemSolution {
        problem(args.0, args.1, args.2)
    }
}

pub fn job_scheduling(start: Vec<i32>, end: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut memo: Vec<i32> = vec![0; start.len()];
    //set the first index since we iterate from idx 1
    memo[0] = profit[0];

    let job = Jobs { start, end, profit };

    job.job_schedule(&mut memo)
}

pub struct Jobs {
    pub start: Vec<i32>,
    pub end: Vec<i32>,
    pub profit: Vec<i32>,
}

impl Jobs {
    fn job_schedule(&self, memo: &mut Vec<i32>) -> i32 {
        let len = self.start.len();
        for i in 1..len {
            let mut p = self.profit[i];
            let mut idx = self.last_biggest_nonconflict(i);
            while idx != -1 {
                p += self.profit[idx as usize];
                idx = self.last_biggest_nonconflict(idx as usize);
            }

            memo[i] = max(p, memo[i - 1]);
        }

        //  memo.sort();
        memo[memo.len() - 1]
    }

    fn last_biggest_nonconflict(&self, index: usize) -> i32 {
        if index == 0 {
            return -1;
        }
        for j in (0..=index - 1).rev() {
            let s1 = self.start[index];
            let e1 = self.end[index];
            let s2 = self.start[j];
            let e2 = self.end[j];

            if s1 == s2 && e1 == e2 {
                continue;
            }

            if s1 >= e2 && e2 < e1 && s1 != s2 {
                //if !(s1 < e2) && !(e2 >= e1) && !(s1 == s2) {

                return j as i32;
            }
        }
        -1
    }
}

/* Assume input is already clean, get rid of checks
pub const LENGTH_UPPER: i32 = 5 * 10 ^ 4;
pub const TIME_UPPER: i32 = 10 ^ 9;
pub const PROFIT_UPPER: i32 = 10 ^ 4;
pub const LOWER: i32 = 1;

fn check_valid_inputs(start: Vec<i32>, end: Vec<i32>, profit: Vec<i32>) -> Result<(), ()> {
    if start.len() != end.len() && start.len() != profit.len() {
        return Err(());
    }
    check_time_valid(start, end)?;
    check_profit_valid(profit)?;
    Ok(())
}

fn check_time_valid(start: Vec<i32>, end: Vec<i32>) -> Result<(), ()> {
    let len = start.len();
    for i in 0..len {
        if start[i] < LOWER || start[i] >= end[i] || end[i] > TIME_UPPER {
            return Err(());
        }
    }
    Ok(())
}

fn check_profit_valid(profit: Vec<i32>) -> Result<(), ()> {
    for amnt in profit {
        if amnt < LOWER || amnt > PROFIT_UPPER {
            return Err(());
        }
    }

    Ok(())
}
*/

//All example cases are pulled from LC
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let start: Vec<i32> = vec![1, 2, 3, 3];
        let end: Vec<i32> = vec![3, 4, 5, 6];
        let profit: Vec<i32> = vec![50, 10, 40, 70];

        let correct = 120;
        assert_eq!(job_scheduling(start, end, profit), correct);
    }

    #[test]
    fn example_2() {
        let start: Vec<i32> = vec![1, 2, 3, 4, 6];
        let end: Vec<i32> = vec![3, 5, 10, 6, 9];
        let profit: Vec<i32> = vec![20, 20, 100, 70, 60];

        let correct = 150;
        assert_eq!(job_scheduling(start, end, profit), correct);
    }

    #[test]
    fn example_3() {
        let start: Vec<i32> = vec![1, 1, 1];
        let end: Vec<i32> = vec![2, 3, 4];
        let profit: Vec<i32> = vec![5, 6, 4];

        let correct = 6;
        assert_eq!(job_scheduling(start, end, profit), correct);
    }
}
