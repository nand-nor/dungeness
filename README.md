# Dungeness: Crabby Practice

Practicing Rust and brushing up on old algorithms/problem solving approaches. Main
focus is on graphs/DP, problem definitions pulled from LC with links where appropriate
(Re: repo name, dungeness is a species of crab :) ) 

### [Regex Match](https://leetcode.com/problems/regular-expression-matching/)

####Status: 
Have implemented regex match using DP/memoization approach. Added some
niceties to reject invalid input before ever running them through the helper memo function.
Would like to also add in a graph impl and maybe for extra fun a finite automata approach?

### [Maximum profit in job scheduling](https://leetcode.com/problems/maximum-profit-in-job-scheduling/)
####Status:
Have implemented using a memoized approach, some trivial test cases pass. Need to
ensure that fail cases (bad input) return appropriately   



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