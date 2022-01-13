
/* LC problem: maximum profit in job scheduling
* https://leetcode.com/problems/maximum-profit-in-job-scheduling/
*/


pub struct JobSched<T: FnMut(Vec<i32>, Vec<i32>,Vec<i32>)->i32> {
    _fn_ptr: T,
}
use crate::problem::Solution;

impl<T: FnMut(Vec<i32>, Vec<i32>,Vec<i32>)->i32> Solution for JobSched<T> {
    type ProblemFunc = T;
    type ProblemArgs = (Vec<i32>, Vec<i32>,Vec<i32>);
    type ProblemSolution = i32;
    fn solution(
        mut problem: Box<Self::ProblemFunc>,
        args: Self::ProblemArgs,
    ) -> Self::ProblemSolution {
        problem(args.0, args.1, args.2)
    }
}

pub fn job_scheduling(start: Vec<i32>, end: Vec<i32>, profit: Vec<i32>)->i32{

}

fn job_scheduling_aux(start: &mut Vec<i32>, end: &mut Vec<i32>, profit: &mut Vec<i32>)->i32{

}