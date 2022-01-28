/// Trait for defining specific solution types of graph solutions for LC problem impls
///
/// Trait types allow the user to provide a function pointer,
/// arguments to said function, and a solution type. User must implement the Solution
/// trait for whatever object this trait is implemented for
///
/// The function impls of this trait will not be possible for all problem specifications, so
/// some may be handled by having a function body of ```unimplemented!()```
/// May at some point break these out into subtraits and have special ones for specific graph
/// algorithm solutions for less common ones like topological sort or euler tour
use crate::Solution;

pub trait GraphSolution: Solution {
    type GraphProblem;
    type GraphProblemArgs;
    type GraphProblemSolution;

    fn solution(
        problem: Box<Self::GraphProblem>,
        args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution;

    fn bfs_solution(
        problem: Box<Self::GraphProblem>,
        args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution;

    fn dfs_solution(
        problem: Box<Self::GraphProblem>,
        args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution;
}

