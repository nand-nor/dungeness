extern crate dungeness;

use dungeness::job_sched;
use dungeness::job_sched::JobSched;
use dungeness::problem::Solution;
use dungeness::regex_match;
use dungeness::regex_match::RegexMatch;

fn main() {
    match RegexMatch::solution(
        std::boxed::Box::new(regex_match::is_match),
        ("aaabbbccc".to_string(), ".*.*c*".to_string()),
    ) {
        true => {
            println!("Pattern returned true, case passes!");
        }
        false => {
            println!("Pattern false");
        }
    }

    match JobSched::solution(
        std::boxed::Box::new(job_sched::job_scheduling),
        (vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
    ) {
        120 => {
            println!("Job scheduling returned optimal answer");
        }
        _ => {
            println!("Job scheduling returned incorrect answer");
        }
    }
}
