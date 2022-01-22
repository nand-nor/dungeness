use crate::Solution;
use std::boxed::Box;
use std::cmp::max;

/// LC problem: maximum profit in job scheduling
/// https://leetcode.com/problems/maximum-profit-in-job-scheduling/
///

pub struct JobSched<T: Fn(Vec<i32>, Vec<i32>, Vec<i32>) -> i32> {
    _fn_ptr: T,
}

/// Solution trait impl for JobSched object
///
/// The function pointer provided to the solution is further defined
/// below, using the Jobs struct object, jobs_scheduling, and Jobs methods
///
impl<T: Fn(Vec<i32>, Vec<i32>, Vec<i32>) -> i32> Solution for JobSched<T> {
    type ProblemFunc = T;
    type ProblemArgs = (Vec<i32>, Vec<i32>, Vec<i32>);
    type ProblemSolution = i32;
    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution {
        problem(args.0, args.1, args.2)
    }
}

pub struct Jobs {
    pub start: Vec<i32>,
    pub end: Vec<i32>,
    pub profit: Vec<i32>,
}

/// The problem function signature as defined by LC spec
///
/// Takes three vectors of i32 and returns i32, using the Jobs struct and
/// Jobs methods to return the greatest possible profit given a series of
/// job schedules
///
pub fn job_scheduling(start: Vec<i32>, end: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut memo: Vec<i32> = vec![0; start.len()];
    //set the first index since we iterate from idx 1
    memo[0] = profit[0];

    let job = Jobs { start, end, profit };

    job.job_schedule(&mut memo)
}

impl Jobs {
    fn job_schedule(&self, memo: &mut Vec<i32>) -> i32 {
        let len = self.start.len();
        for i in 1..len {
            let mut p = self.profit[i];
            let mut idx = self.last_nonconflict(i);
            while let Some(index) = idx {
                p += self.profit[index];
                idx = self.last_nonconflict(index);
            }
            memo[i] = max(p, memo[i - 1]);
        }
        memo[memo.len() - 1]
    }

    fn last_nonconflict(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
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
                return Some(j);
            }
        }
        None
    }
}

/// All example test cases are pulled from LC
/// Need to test more edge cases
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
