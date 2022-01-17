/* Make a generic Graph Solution trait for LC problem impls */

pub trait GraphSolution {
    type GraphProblem;
    type GraphProblemArgs;
    type GraphProblemSolution;
    fn solution(
        problem: Box<Self::GraphProblem>,
        args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution;
}
