extern crate dungeness;
#[macro_use]
extern crate bencher;

use bencher::Bencher;

use dungeness::cheapestk;
use dungeness::cheapestk::{CheapestFlight, CheapestFlightWrap};
use dungeness::graph::{GraphSolution, GraphSolutionType};
//use dungeness::graph::GraphSolution;
use dungeness::job_sched;
use dungeness::job_sched::JobSched;
use dungeness::regex_match;
use dungeness::regex_match::RegexMatch;
use dungeness::{BenchSolution, Solution, SolutionForm};

fn bench_regex_match_problem(b: &mut Bencher) {
    b.iter(|| {
        bencher::black_box(RegexMatch::solution(
            std::boxed::Box::new(regex_match::is_match),
            ("aaabbbccc".to_string(), ".*.*c*".to_string()),
        ))
    });
}

fn bench_job_schedule_problem(b: &mut Bencher) {
    b.iter(|| {
        bencher::black_box(JobSched::solution(
            std::boxed::Box::new(job_sched::job_scheduling),
            (
                vec![1, 1, 2, 3, 3, 5, 6],
                vec![2, 3, 4, 5, 6, 7, 9],
                vec![50, 45, 10, 40, 70, 25, 80],
            ),
        ))
    });
}

fn bench_cheapestk_graph_bfs(b: &mut Bencher) {
    let n = 3;
    let src = 0;
    let dst = 2;
    let k = 1;

    b.iter(|| {
        let cheapo = CheapestFlight::new(cheapestk::bfs_find_cheapest_price);

        bencher::black_box(
            cheapo.bfs_solution(
            (
                n,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                src,
                dst,
                k,
            ),
        ))
    });
}

fn bench_cheapestk_dp(b: &mut Bencher) {
    let n = 3;
    let src = 0;
    let dst = 2;
    let k = 1;

    b.iter(|| {
        bencher::black_box(<CheapestFlight<_> as Solution>::solution(
            std::boxed::Box::new(cheapestk::find_cheapest_price),
            (
                n,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                src,
                dst,
                k,
            ),
        ))
    });
}

/// Test absurd trait implementation
/// Measure cost of vtable lookups, indirection through boxes, etc.

fn bench_cheapestk_bfs_trait(b: &mut Bencher) {
    let n = 3;
    let src = 0;
    let dst = 2;
    let k = 1;

    b.iter(|| {

        let cheapo_wrap = CheapestFlightWrap::default();

        bencher::black_box(cheapo_wrap.bench_solution(
            CheapestFlight::new(std::boxed::Box::new(cheapestk::bfs_find_cheapest_price)),
            GraphSolutionType::BFS,
            (
                n,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                src,
                dst,
                k,
            ),
        ))
    });
}

benchmark_group!(
    benches,
    bench_regex_match_problem,
    bench_job_schedule_problem,
    bench_cheapestk_graph_bfs,
    bench_cheapestk_dp,
    bench_cheapestk_bfs_trait,
);
benchmark_main!(benches);
