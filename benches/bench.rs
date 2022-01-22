extern crate dungeness;
#[macro_use]
extern crate bencher;

use bencher::Bencher;

use dungeness::job_sched;
use dungeness::job_sched::JobSched;
use dungeness::regex_match;
use dungeness::regex_match::RegexMatch;
use dungeness::Solution;

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

benchmark_group!(
    benches,
    bench_regex_match_problem,
    bench_job_schedule_problem
);
benchmark_main!(benches);
