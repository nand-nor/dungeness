extern crate itertools;

pub mod dp;
pub mod graph;

pub mod cheapestk;
pub mod job_sched;
pub mod regex_match;
pub mod skyline;
pub mod stones;

/// Trait for defining generic solution for LC problem impls
///
/// Trait types allow the user to provide a function pointer,
/// arguments to said function, and a solution type.
///
/// # Examples
///
/// ```
/// use dungeness::Solution;
/// struct Answer<T: Fn(Vec<f64>)->bool> {
///     fptr: T
/// }
/// impl <T: Fn(Vec<f64>)->bool> Solution for Answer<T> {
///     type ProblemFunc= T;
///     type ProblemArgs= Vec<f64>;
///     type ProblemSolution= bool;
///
///    fn solution(problem: Box<Self::ProblemFunc>,args: Self::ProblemArgs) -> Self::ProblemSolution {
///         problem(args)
///     }
/// }
///
///
/// ```
pub trait Solution {
    //type ProblemType: BenchSolution;
    type ProblemFunc;
    type ProblemArgs;
    type ProblemSolution;

    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution;
}

pub trait SolutionForm {
    type ProblemSubtype;
    type ProblemFunc;
    type ProblemArgs;
    type ProblemSolution;
    fn do_solution<T: BenchSolution>(
        &self,
        form: &T,
        subtype: Self::ProblemSubtype,
     //   problem: Box<Self::ProblemFunc>,
        args: Self::ProblemArgs,
    ) -> Self::ProblemSolution;
}

pub trait BenchSolution: Sized {
    type ProblemType: SolutionForm;
    type ProblemSubtype;
    type ProblemFunc;
    type ProblemArgs;
    type ProblemSolution;

    fn bench_solution(
        &self,
        sol_type: Self::ProblemType,
        sol_subtype: <<Self as BenchSolution>::ProblemType as SolutionForm>::ProblemSubtype,//Self::ProblemSubtype,
      //  problem: Box<<<Self as BenchSolution>::ProblemType as SolutionForm>::ProblemFunc>,//Box<Self::ProblemFunc>,
        args: <<Self as BenchSolution>::ProblemType as SolutionForm>::ProblemArgs,//Self::ProblemArgs,
    ) -> <<Self as BenchSolution>::ProblemType as SolutionForm>::ProblemSolution//<Self as BenchSolution>::ProblemSolution
    where
        Self: Sized{
            sol_type.do_solution(self, sol_subtype, /*problem,*/ args) // as <Self as BenchSolution>::ProblemSolution


    }
}

#[cfg(test)]
mod tests {
    use crate::job_sched::{job_scheduling, JobSched};
    use crate::regex_match::{is_match, RegexMatch};
    use crate::Solution;
    // extern crate bencher;
    // use bencher::Bencher;

    #[test]
    fn solution_impl_test() {
        assert!(RegexMatch::solution(
            std::boxed::Box::new(is_match),
            ("aaabbbccc".to_string(), ".*.*c*".to_string())
        ));

        assert_eq!(
            JobSched::solution(
                std::boxed::Box::new(job_scheduling),
                (vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            ),
            120
        );
    }
}
