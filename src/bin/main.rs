extern crate dungeness;

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
}
