use std::boxed::Box;
use std::cmp::max;

use crate::Solution;

/// LC problem: stone game ii & iii
/// https://leetcode.com/problems/stone-game-ii/
/// https://leetcode.com/problems/stone-game-iii/


pub struct StoneGameII<T: Fn(Vec<i32>) -> i32> {
    _fn_ptr: T,
}

impl<T: Fn(Vec<i32>) -> i32> Solution for StoneGameII<T> {
    type ProblemFunc = T;
    type ProblemArgs = Vec<i32>;
    type ProblemSolution = i32;
    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution {
        problem(args)
    }
}

struct Stones {
    sums: Vec<i32>,
    memo: Vec<Vec<i32>>,
}

pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let len = piles.len();
    let dpmemo = vec![vec![-1;len+1]; len + 1];

    let len = piles.len();

    let mut sums = vec![0;len];
    sums[len-1] = piles[len-1];

    for i in (0..=len - 2).rev(){
        sums[i] = sums[i+1] + piles[i];
    }

    let stones = Stones::new(sums, dpmemo);
    stones.stone_game_ii_aux(0,1)
}

impl Stones {
    fn new(sums: Vec<i32>, memo: Vec<Vec<i32>>)->Self{
        Self{
            sums,
            memo
        }
    }
    fn stone_game_ii_aux(&self, idx: isize, current_m: isize) -> i32 {
        if idx >= self.sums.len() as isize{
            return 0
        } else if self.memo[idx as usize][current_m as usize] != -1 {
            return self.memo[idx as usize][current_m as usize]
        }else if (idx - 1 + 2 * current_m) >= self.sums.len() as isize {
            return self.sums[idx as usize]
        }
            let mut maxp: i32 = 1<<31;

        for i in 1..(2 * current_m + 1) {
            maxp = max(maxp, self.sums[idx as usize] - self.stone_game_ii_aux(idx + i, max(i, current_m)));
        }

        maxp
    }
}

/*
pub struct StoneGameIII<T: Fn(Vec<i32>) -> String> {
    _fn_ptr: T,
}

impl<T: Fn(Vec<i32>) -> String> Solution for StoneGameIII<T> {
    type ProblemFunc = T;
    type ProblemArgs = Vec<i32>;
    type ProblemSolution = String;
    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution {
        problem(args)
    }
}

pub fn stone_game_iii(piles: Vec<i32>) -> String {
    "Bob".to_string()
}

fn stone_game_iii_aux(piles: Vec<i32>, memo: Vec<i32>)->String{
    "Bob".to_string()
}
*/

/// test functions named as sg*_example_* are pulled directly from LC
/// Additional edge cases to be added
#[cfg(test)]
mod tests{
    use crate::stones::*;
    #[test]
    fn sgii_example_1(){
        let input = vec![2,7,9,4,4];
        let sol = 10;

        assert_eq!(stone_game_ii(input), sol);
    }
    #[test]
    fn sgii_example_2(){
        let input = vec![1,2,3,4,5,100];
        let sol = 104;

        assert_eq!(stone_game_ii(input), sol);
    }

    /*
    #[test]
    fn sgiii_example_1(){
        let input = vec![1,2,3,7];
        let sol = "Bob";

        assert_eq!(stone_game_iii(input), sol);
    }
    #[test]
    fn sgiii_example_2(){
        let input = vec![1,2,3,-9];
        let sol = "Alice";

        assert_eq!(stone_game_iii(input), sol);
    }*/
}