/// Trait for defining specific solution types of dynamic programming solutions for LC problem impls
///
/// Trait types allow the user to provide a function pointer,
/// arguments to said function, and a solution type. User must implement the Solution
/// trait for whatever object this trait is implemented for
///

pub trait DynProgSolution {
    type DynProgProblemArgs;
    type DynProgProblemSolution;

    fn memo_solution(&self,
        args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution
    where
        Self: Sized;

    fn bottomup_solution(&self,
        args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution
    where
        Self: Sized;

    fn topdown_solution(&self,
        args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution
    where
        Self: Sized;
}

pub trait CustomDynProgSolutionType {}

pub enum DynProgSolutionType {
    TOP,
    BOTTOM,
    MEMO,
    CUSTOM(Box<dyn CustomDynProgSolutionType>),
}

