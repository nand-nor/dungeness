
/* LC problem: skyline problem
 * https://leetcode.com/problems/the-skyline-problem/
*/


pub struct Skyline<T: Fn(Vec<Vec<i32>>) -> Vec<Vec<i32>>> {
    _fn_ptr: T,
}

use crate::problem::Solution;

impl<T: Fn(Vec<Vec<i32>>) -> Vec<Vec<i32>>> Solution for Skyline<T> {
    type ProblemFunc = T;
    type ProblemArgs = Vec<Vec<i32>>;
    type ProblemSolution = Vec<Vec<i32>>;
    fn solution(
        problem: Box<Self::ProblemFunc>,
        args: Self::ProblemArgs,
    ) -> Self::ProblemSolution {
        problem(args)
    }
}
