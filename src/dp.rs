/* Make a generic DP Solution trait for LC problem impls */

pub trait DynamicProgrammingSolution {
    type DynamicProgrammingProblem;
    type DynamicProgrammingProblemArgs;
    type DynamicProgrammingProblemSolution;
    fn solution(
        problem: Box<Self::DynamicProgrammingProblem>,
        args: Self::DynamicProgrammingProblemArgs,
    ) -> Self::DynamicProgrammingProblemSolution;
}
