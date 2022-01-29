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

pub trait GraphSolution {
    type GraphProblemArgs;
    type GraphProblemSolution;

    fn bfs_solution(
        &self,
        args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution
    where
        Self: Sized;

    fn dfs_solution(
        &self,
        args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution
    where
        Self: Sized;
}


pub trait CustomGraphSolution {}

pub enum GraphSolutionType {
    BFS,
    DFS,
    OTHER(Box<dyn CustomGraphSolution>),
}
