/// Trait for defining specific solution types of dynamic programming solutions for LC problem impls
///
/// Trait types allow the user to provide a function pointer,
/// arguments to said function, and a solution type. User must implement the Solution
/// trait for whatever object this trait is implemented for
///
use crate::Solution;

pub trait DynProgSolution: Solution {
    type DynProgProblem;
    type DynProgProblemArgs;
    type DynProgProblemSolution;

    fn memo_solution(
        problem: Box<Self::DynProgProblem>,
        args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution;

    fn bottomup_solution(
        problem: Box<Self::DynProgProblem>,
        args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution;

    fn topdown_solution(
        problem: Box<Self::DynProgProblem>,
        args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution;
}
