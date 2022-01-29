use crate::dp::DynProgSolution;
use crate::graph::{GraphSolution, GraphSolutionType};

use crate::Solution;
use std::boxed::Box;
use std::cmp::min;
use std::collections::VecDeque;

/// LC problem: Cheapest Flights within K Stops
/// https://leetcode.com/problems/cheapest-flights-within-k-stops/
///


pub struct CheapestFlight<T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32 + ?Sized> {
    _fn_ptr: Box<T>,
}

impl<T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32> Solution for CheapestFlight<T> {
    type ProblemFunc = T;
    type ProblemArgs = (i32, Vec<Vec<i32>>, i32, i32, i32);
    type ProblemSolution = i32;
    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution {
        problem(args.0, args.1, args.2, args.3, args.4)
    }
}

impl <T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32>CheapestFlight<T>{
    pub fn new(t: T)->Self{
        Self{
            _fn_ptr: Box::new(t),
        }
    }
}

struct CheapestDP {
    memo: Vec<Vec<i32>>,
    max: i32,
}

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut dp = CheapestDP::new(n, k);
    dp.cheapest_flight_aux(flights, src, dst, k)
}

impl CheapestDP {
    pub fn new(n: i32, k: i32) -> Self {
        //per problem spec, all prices will be less than or equal to 10^4
        let max_price = 10_i32.pow(4);
        let memo = vec![vec![max_price + 1; n as usize]; k as usize + 2];

        //store both the max price (will use for price checking later)
        //and the memo matrix in the CheapestDP object
        Self {
            memo,
            max: max_price,
        }
    }

    fn cheapest_flight_aux(&mut self, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        self.memo[0][src as usize] = 0;
        for idx in 1..(k as usize + 2) {
            self.memo[idx][src as usize] = 0;
            for (node_u, node_v, price) in flights.iter().map(|node| (node[0], node[1], node[2])) {
                self.memo[idx][node_v as usize] = min(
                    self.memo[idx][node_v as usize],
                    self.memo[idx - 1][node_u as usize] + price,
                );
            }
        }
        if self.memo[k as usize + 1][dst as usize] != self.max {
            self.memo[k as usize + 1][dst as usize]
        } else {
            -1
        }
    }
}

impl<T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32> DynProgSolution for CheapestFlight<T>
where
    CheapestFlight<T>: Solution,
{
    type DynProgProblemArgs = (i32, Vec<Vec<i32>>, i32, i32, i32);
    type DynProgProblemSolution = i32;

    fn memo_solution(&self,
        args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution {
        (self._fn_ptr)(args.0, args.1, args.2, args.3, args.4)
    }

    fn bottomup_solution(&self,
        _args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution {
        unimplemented!();
    }

    fn topdown_solution(&self,
        _args: Self::DynProgProblemArgs,
    ) -> Self::DynProgProblemSolution {
        unimplemented!();
    }
}

struct CheapestBFS {
    graph: Vec<Vec<bool>>, //adjacency matrix
    queue: VecDeque<(i32, i32)>,
    flights: Vec<Vec<i32>>,
}

pub fn bfs_find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let mut dp = CheapestBFS::new(n, flights, src);
    dp.cheapest_flight_aux(dst, k)
}

impl CheapestBFS {
    fn new(n: i32, flights: Vec<Vec<i32>>, problem_src: i32) -> Self {
        let mut graph = vec![vec![false; n as usize]; n as usize];
        let mut queue = VecDeque::new();
        for (idx, node) in graph.iter_mut().enumerate() {
            for (src, dst, price) in flights.iter().map(|node| (node[0], node[1], node[2])) {
                if src as usize == idx {
                    node[dst as usize] = true;
                    //if this node is a neighbor to the problem source, enqueue it
                    if idx == problem_src as usize {
                        queue.push_back((dst, price));
                    }
                }
            }
        }

        Self {
            graph,
            queue,
            flights,
        }
    }

    fn cheapest_flight_aux(&mut self, dst: i32, k: i32) -> i32 {
        let mut cheapest = -1;

        for _idx in 0..k as usize + 1 {
            let size = self.queue.len();

            for _j in 0..size {
                if let Some((node_v, price)) = self.queue.pop_front() {
                    if node_v == dst {
                        cheapest = {
                            if cheapest == -1 {
                                price
                            } else {
                                min(cheapest, price)
                            }
                        };
                    } else {
                        for neighbor in self.graph[node_v as usize].iter() {
                            //if its a neighbor, enqueue it with cumulative price
                            if *neighbor {
                                let (ndst, nprice) = (
                                    self.flights[node_v as usize][1],
                                    self.flights[node_v as usize][2],
                                );
                                self.queue.push_back((ndst, price + nprice));
                            }
                        }
                    }
                }
            }
        }
        cheapest
    }
}

use crate::{BenchSolution, SolutionForm};

impl<T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32> SolutionForm for CheapestFlight<T> {
    type ProblemSubtype = GraphSolutionType;
    type ProblemFunc = T;
    type ProblemArgs = (i32, Vec<Vec<i32>>, i32, i32, i32);
    type ProblemSolution = i32;
    fn do_solution<S: BenchSolution>(
        &self,
        form: &S,
        subtype: Self::ProblemSubtype,
        args: Self::ProblemArgs,
    ) -> Self::ProblemSolution {

        match subtype {
            GraphSolutionType::BFS=>{
                self.bfs_solution(args)
            }
            GraphSolutionType::DFS=>{
                unimplemented!()
            }
            _=>{
                unimplemented!()
            }
        }
    }
}

#[derive(Default)]
pub struct CheapestFlightWrap{}

impl BenchSolution for CheapestFlightWrap{

    type ProblemType = CheapestFlight<Box<dyn Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32>>;
    type ProblemSubtype = GraphSolutionType;
    type ProblemFunc = Box<dyn Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32>;
    type ProblemArgs = (i32, Vec<Vec<i32>>, i32, i32, i32);
    type ProblemSolution = i32;
}


impl<T: Fn(i32, Vec<Vec<i32>>, i32, i32, i32) -> i32> GraphSolution for CheapestFlight<T>
where
    CheapestFlight<T>: Solution,
{
    type GraphProblemArgs = (i32, Vec<Vec<i32>>, i32, i32, i32);
    type GraphProblemSolution = i32;

    fn bfs_solution(
        &self,
        args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution {
        (self._fn_ptr)(args.0, args.1, args.2, args.3, args.4)
    }

    fn dfs_solution(
        &self,
        _args: Self::GraphProblemArgs,
    ) -> Self::GraphProblemSolution {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use crate::cheapestk::bfs_find_cheapest_price;
    use crate::cheapestk::find_cheapest_price;

    #[test]
    fn dp_single_stop() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        let sol = 200;
        assert_eq!(find_cheapest_price(n, flights, src, dst, k), sol);
    }

    #[test]
    fn dp_zero_stops() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        let sol = 500;
        assert_eq!(find_cheapest_price(n, flights, src, dst, k), sol);
    }

    #[test]
    fn bfs_single_stop() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        let sol = 200;
        assert_eq!(bfs_find_cheapest_price(n, flights, src, dst, k), sol);
    }

    #[test]
    fn bfs_zero_stops() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        let sol = 500;

        assert_eq!(bfs_find_cheapest_price(n, flights, src, dst, k), sol);
    }
}
