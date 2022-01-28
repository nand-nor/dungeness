use std::boxed::Box;
use std::cmp::min;
use crate::Solution;

/// LC problem: Cheapest Flights within K Stops
/// https://leetcode.com/problems/cheapest-flights-within-k-stops/
///

pub const PATTERN_UPPER: usize = 30;
pub const INPUT_UPPER: usize = 20;
pub const LOWER: usize = 1;

pub struct CheapestFlight<T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32> {
    _fn_ptr: T,
}

impl<T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32> Solution for CheapestFlight<T> {
    type ProblemFunc = T;
    type ProblemArgs = (i32, Vec<Vec<i32>>, i32, i32, i32);
    type ProblemSolution = i32;
    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution {
        problem(args.0, args.1, args.2, args.3, args.4)
    }
}

struct CheapestDP{
    memo: Vec<Vec<i32>>,
    max: i32,
}

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let len= flights.len();

    let mut dp = CheapestDP::new(n,k);

    dp.cheapest_flight_aux(n,flights,src,dst,k)
}



impl CheapestDP {
    pub fn new(n: i32, k: i32)->Self{
        //per problem spec, all prices will be less than or equal to 10^4
        let max_price = 10_i32.pow(4);
        let mut memo = vec![vec![max_price+1; n as usize]; k as usize +2];

        //store both the max price (will use for price checking later)
        //and the memo matrix in the CheapestDP object
        Self{
            memo,
            max: max_price,
        }
    }

    fn cheapest_flight_aux(&mut self, n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32{
        self.memo[0][src as usize] = 0;
        for idx in 1..(k as usize + 2){
            self.memo[idx][src as usize] = 0;
            for (node_u, node_v, price) in flights.iter().map(|node|(node[0],node[1],node[2])){
                self.memo[idx][node_v as usize] = min(self.memo[idx][node_v as usize], self.memo[idx - 1][node_u as usize] + price);
            }
        }
        if self.memo[k as usize+1][dst as usize] != self.max {
            return self.memo[k as usize+1][dst as usize]
        } else {
            return -1
        }

    }
}



#[cfg(test)]
mod tests {
    use crate::cheapestk::find_cheapest_price;

    #[test]
    fn example_1_single_stop(){
        let n = 3;
        let flights = vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        let sol = 200;
        assert_eq!(find_cheapest_price(n,flights,src,dst,k),sol);
    }

    #[test]
    fn example_2_zero_stops(){
        let n = 3;
        let flights = vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        let sol = 500;
        assert_eq!(find_cheapest_price(n,flights,src,dst,k),sol);
    }
}