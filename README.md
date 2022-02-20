# Dungeness: Crabby Practice and Solution Benchmarking

Practicing Rust, brushing up on old algorithms/problem solving approaches, and benchmarking solution approaches.

The ultimate goal of this repo is to practice Rust and algorithms, with a side goal of investigating different approaches via benchmarking solutions.
Also looking into efficient (and not so efficient) solution trait implementations.

Main focus for solution benchmarking/practicing is on graphs/DP, problem definitions pulled from LC with links where appropriate
(Re: repo name, dungeness is a species of crab :) :crystal_ball: :ghost: ) 



### [Regex Match](https://leetcode.com/problems/regular-expression-matching/)

#### Status: 
Have implemented regex match using DP/memoization approach. Added some
niceties to reject invalid input before ever running them through the helper memo function.
Would like to also add in a graph impl and maybe for extra fun a finite automata approach?

### [Maximum Profit in Job Scheduling](https://leetcode.com/problems/maximum-profit-in-job-scheduling/)

#### Status:
Have implemented using a memoized approach, some trivial test cases pass. Need to
ensure that fail cases (bad input) return appropriately   

### [Skyline Problem](https://leetcode.com/problems/the-skyline-problem/)

#### Status:
Have implemented a topdown DP approach, needs some refactoring and other impls for full benchmarking

### [Cheapest Flights in K stops]( https://leetcode.com/problems/cheapest-flights-within-k-stops/)

#### Status:
Have implemented a graph solution (BFS) and a DP solution (memoization). 


## Benchmarks

Work in progress, see `benches/bench.rs`

## TO DO
- Using `Solution` as a super trait, want to implement DP and graph-specific solution sub traits,
so memoization, top down or bottom up for DP, and things like BFS and DFS for graph solutions. 
- Once multiple solution types are implemented, add more comprehensive benchmarks. Want to compare
 the performance using `bencher`. 
- More DP hard level problems ! And more graph problems (my favorite) :
    - [Set intersection size (at least two)](https://leetcode.com/problems/set-intersection-size-at-least-two/)
    - [Kth smallest product of two sorted arrays](https://leetcode.com/problems/kth-smallest-product-of-two-sorted-arrays/)
    - [Skyline problem](https://leetcode.com/problems/the-skyline-problem/)
    - [Max edge removal](https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/)
    - [Largest color in directed graph](https://leetcode.com/problems/largest-color-value-in-a-directed-graph/)