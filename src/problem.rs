
/* Make a generic trait for LC problem impls */

pub trait Solution {
    type ProblemFunc;
    type ProblemArgs;
    type ProblemSolution;

    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution;
}
