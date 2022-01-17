use std::boxed::Box;

use itertools::Itertools;

/* LC problem: Regular Expression Matching
* https://leetcode.com/problems/regular-expression-matching/
*/

pub const PATTERN_UPPER: usize = 30;
pub const INPUT_UPPER: usize = 20;
pub const LOWER: usize = 1;

pub struct RegexMatch<T: FnMut(String, String) -> bool> {
    _fn_ptr: T,
}
use crate::problem::Solution;

impl<T: FnMut(String, String) -> bool> Solution for RegexMatch<T> {
    type ProblemFunc = T;
    type ProblemArgs = (String, String);

    type ProblemSolution = bool;
    fn solution(
        mut problem: Box<Self::ProblemFunc>,
        args: Self::ProblemArgs,
    ) -> Self::ProblemSolution {
        problem(args.0, args.1)
    }
}

pub fn is_match(s: String, p: String) -> bool {
    match check_input_valid(s.clone()) {
        Ok(()) => {}
        Err(()) => {
            return false;
        }
    }
    match check_pattern_valid(p.clone()) {
        Ok(has_kleene) => {
            if !has_kleene && p.len() > s.len() {
                return false;
            }
        }
        Err(()) => {
            return false;
        }
    }

    let rows = s.len() + 1;
    let cols = p.len() + 1;
    let mut memos: Vec<Vec<Option<bool>>> = vec![vec![None; rows]; cols];
    memoized_match(s.as_bytes(), p.as_bytes(), 0, 0, &mut memos)
}

pub fn memoized_match(
    input: &[u8],
    pattern: &[u8],
    row: usize,
    column: usize,
    matrix: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    let res: bool;

    //Handle all possibilities for invalid accesses and be sure to return correct ansert in case
    // kleenes are in the final position or in the second position (will be rejected as invalid otherwise)

    match pattern.len().cmp(&input.len()) {
        std::cmp::Ordering::Greater => {
            if row >= input.len() - 1 {
                return true;
            }
        }
        std::cmp::Ordering::Less => {
            if row >= pattern.len() && pattern[pattern.len() - 1] == 0x2a {
                return pattern[pattern.len() - 2] == 0x2e
                    || input[input.len() - 1] == pattern[pattern.len() - 2];
            } else if pattern[pattern.len() - 1] != 0x2a {
                return false;
            }
        }
        _ => {}
    }

    match matrix[row][column] {
        None => {
            res = {
                if column == pattern.len() {
                    row == input.len()
                } else {
                    let first_match: bool = row < input.len()
                        && (pattern[column] == input[row] || pattern[column] == 0x2e);
                    if (input[row] == pattern[column] || pattern[column] == 0x2e)
                        && row > 0
                        && column > 0
                    {
                        if let Some(_answer) = matrix[row - 1][column - 1] {
                            matrix[row][column] = Some(true);
                        }
                    }
                    if column + 1 < pattern.len() && pattern[column + 1] == 0x2a {
                        if !memoized_match(input, pattern, row, column + 2, matrix) {
                            let res1 = memoized_match(input, pattern, row + 1, column, matrix);
                            matrix[row][column] = Some(first_match && res1);
                            first_match && res1
                        } else {
                            matrix[row][column] = Some(true);
                            true
                        }
                    } else {
                        let res1 = memoized_match(input, pattern, row + 1, column + 1, matrix);
                        matrix[row][column] = Some(first_match && res1);
                        first_match && res1
                    }
                }
            };
            res
        }
        Some(answer) => return answer,
    };
    res
}

#[allow(clippy::single_match, clippy::result_unit_err)]
pub fn check_pattern_valid(pattern: String) -> Result<bool, ()> {
    check_len(pattern.len(), PATTERN_UPPER, LOWER)?;
    pattern
        .bytes()
        .into_iter()
        .try_for_each(|c| -> Result<(), ()> {
            match c {
                97..=122 => Ok(()),
                0x2a..=0x2a => Ok(()),
                0x2e..=0x2e => Ok(()),
                _ => Err(()),
            }
        })?;

    check_kleene_valid(pattern)
}
#[allow(clippy::result_unit_err)]
pub fn check_len(len: usize, upper: usize, lower: usize) -> Result<(), ()> {
    if lower <= len && upper >= len {
        return Ok(());
    }
    Err(())
}

#[allow(clippy::single_match, clippy::result_unit_err)]
pub fn check_input_valid(input: String) -> Result<(), ()> {
    check_len(input.len(), INPUT_UPPER, LOWER)?;
    input
        .bytes()
        .into_iter()
        .try_for_each(|c| -> Result<(), ()> {
            match c {
                97..=122 => Ok(()),
                _ => Err(()),
            }
        })?;
    Ok(())
}

// Ensure that kleene star useage is valid given the contraints of the problem
// Specifically, the problem states: "for each appearance of the character '*',
// there will be a previous valid character to match." So check that this is true,
// mostly just making sure there is no "**" substring in the pattern or a pattern of "*"
#[allow(clippy::single_match, clippy::result_unit_err)]
pub fn check_kleene_valid(pattern: String) -> Result<bool, ()> {
    let mut bytes: Vec<u8> = pattern.bytes()./*map(|x| x).*/collect();
    let mut itr = bytes.iter_mut().multipeek();
    let mut first_itr = true;
    let mut has_kleene: bool = false;
    while let Some(first) = itr.next() {
        if let Some(second) = itr.peek() {
            match second {
                0x2a..=0x2a => {
                    has_kleene = true;
                    match first {
                        0x2a..=0x2a => return Err(()),
                        _ => {}
                    }
                }
                _ => {}
            }
        } else if first_itr {
            match first {
                0x2a..=0x2a => return Err(()),
                _ => {}
            }
        }
        first_itr = false;
    }

    Ok(has_kleene)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::regex_match::*;
    //const VALID_PATTERN: &str = ".*";
    const INPUT_FAIL_CASE_NUM: usize = 7;
    const INPUT_FAIL_CASES: [(&str, &str); INPUT_FAIL_CASE_NUM] = [
        ("alakigifanchtheificdicic", ".*"),
        (".*", ".*"),
        ("BOING", ".*"),
        ("", ".*"),
        ("`", ".*"),
        ("{", ".*"),
        ("input has spaces", ".*"),
    ];

    const PATTERN_FAIL_CASE_NUM: usize = 7;
    const PATTERN_FAIL_CASES: [(&str, &str); PATTERN_FAIL_CASE_NUM] = [
        ("aaa", "alakpatternpattrnpatternpatternisveryverylongcicic"),
        ("inputstring", "pattern has spaces"),
        ("bbbccc", "PATTERNHASUPPER"),
        ("oioioioi", ""),
        ("substring", "{"),
        ("inputstring", "`"),
        ("aaabbbccc", "*"),
    ];

    const FAIL_CASE_NUM: usize = 3;
    const FAIL_CASES: [(&str, &str); FAIL_CASE_NUM] = [
        ("bbb", "c*b"),
        ("aaaabbbbcccc", "aaaabbbbccc"),
        ("aaaabbbbcccc", "a*b*c"),
    ];

    const PASS_CASE_NUM: usize = 5;
    const PASS_CASES: [(&str, &str); PASS_CASE_NUM] = [
        ("boink", ".*"),
        ("aaaabbbbcccc", "aaaabbbbcccc"),
        ("abc", "c*a.c"),
        ("aaaabbbbcccc", "a*b*c*"),
        ("aaaabbbbccc", ".*.*c*"),
    ];
    #[test]
    fn run_bad_input() {
        for (s, p) in INPUT_FAIL_CASES {
            assert!(!RegexMatch::solution(
                std::boxed::Box::new(is_match),
                (s.to_string(), p.to_string())
            ));
        }

        for (s, p) in PATTERN_FAIL_CASES {
            assert!(!RegexMatch::solution(
                std::boxed::Box::new(is_match),
                (s.to_string(), p.to_string())
            ));
        }
    }

    #[test]
    fn run_fails() {
        for (s, p) in FAIL_CASES {
            assert!(!RegexMatch::solution(
                std::boxed::Box::new(is_match),
                (s.to_string(), p.to_string())
            ));
        }
    }

    #[test]
    fn run_passes() {
        for (s, p) in PASS_CASES {
            assert!(RegexMatch::solution(
                std::boxed::Box::new(is_match),
                (s.to_string(), p.to_string())
            ));
        }
    }
}
