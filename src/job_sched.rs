use std::boxed::Box;

//use itertools::Itertools;


/* LC problem: maximum profit in job scheduling
* https://leetcode.com/problems/maximum-profit-in-job-scheduling/
*/
pub const LENGTH_UPPER: i32 = 5*10^4;
pub const TIME_UPPER: i32 = 10^9;
pub const PROFIT_UPPER: i32 = 10^4;
pub const LOWER: i32 = 1;

pub struct JobSched<T: FnMut(Vec<i32>, Vec<i32>,Vec<i32>)->i32> {
    _fn_ptr: T,
}
use crate::problem::Solution;

impl<T: FnMut(Vec<i32>, Vec<i32>,Vec<i32>)->i32> Solution for JobSched<T> {
    type ProblemFunc = T;
    type ProblemArgs = (Vec<i32>, Vec<i32>,Vec<i32>);
    type ProblemSolution = i32;
    fn solution(
        mut problem: Box<Self::ProblemFunc>,
        args: Self::ProblemArgs,
    ) -> Self::ProblemSolution {
        problem(args.0, args.1, args.2)
    }
}

pub fn job_scheduling(start: Vec<i32>, end: Vec<i32>, profit: Vec<i32>)->i32{
    match check_valid_inputs(start,end,profit){
        Ok(())=>{},
        Err(())=>{
            return 0
        }
    }


    0
}

fn job_scheduling_aux(start: &mut Vec<i32>, end: &mut Vec<i32>, profit: &mut Vec<i32>)->i32{
    0
}


fn check_valid_inputs(start: Vec<i32>, end: Vec<i32>, profit: Vec<i32>)->Result<(),()>{
    if start.len() != end.len() && start.len() != profit.len() {
        return Err(())
    }
    check_time_valid(start,end)?;
    check_profit_valid(profit)?;
    Ok(())
}

fn check_time_valid(start: Vec<i32>, end: Vec<i32>)->Result<(),()>{
    let len = start.len();
    for i in 0..len {
            if start[i] < LOWER || start[i] >= end[i] || end[i]  > TIME_UPPER {
                return Err(())
            }
        //}
    }
    Ok(())
}

fn check_profit_valid(profit: Vec<i32>)->Result<(),()>{
    for amnt in profit {
        if amnt  < LOWER || amnt  > PROFIT_UPPER {
            return Err(())
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1(){
        let start: Vec<i32> = vec![1,2,3,3];
        let end: Vec<i32> = vec![3,4,5,6];
        let profit: Vec<i32> = vec![50,10,40,70];

        let correct = 120;
        assert_eq!(job_scheduling(start,end,profit),correct);
    }

    #[test]
    fn example_2(){
        let start: Vec<i32> = vec![1,2,3,4,6];
        let end: Vec<i32> = vec![3,5,10,6,9];
        let profit: Vec<i32> = vec![20,20,100,70,60];

        let correct = 150;
        assert_eq!(job_scheduling(start,end,profit),correct);
    }


    #[test]
    fn example_3(){
        let start: Vec<i32> = vec![1,1,1];
        let end: Vec<i32> = vec![2,3,4];
        let profit: Vec<i32> = vec![5,6,4];

        let correct = 6;
        assert_eq!(job_scheduling(start,end,profit),correct);
    }


}
