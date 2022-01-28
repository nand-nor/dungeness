use crate::Solution;

/// Skyline object for implementing solution to LC problem:
/// https://leetcode.com/problems/the-skyline-problem/
///
/// Wrapper struct with a generic function pointer object
/// to pass as ProblemFunc type to the Solution trait
pub struct Skyline<T: Fn(Vec<Vec<i32>>) -> Vec<Vec<i32>>> {
    _fn_ptr: T,
}

/// Solution trait impl for Skyline object
///
/// The function pointer provided to the solution is further defined
/// below
///
impl<T: Fn(Vec<Vec<i32>>) -> Vec<Vec<i32>>> Solution for Skyline<T> {
    type ProblemFunc = T;
    type ProblemArgs = Vec<Vec<i32>>;
    type ProblemSolution = Vec<Vec<i32>>;
    fn solution(problem: Box<Self::ProblemFunc>, args: Self::ProblemArgs) -> Self::ProblemSolution {
        problem(args)
    }
}

pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //TODO refactor topdown impl, implement memoization as well
    topdown_skyline_aux(buildings)
}

fn topdown_skyline_aux(b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut points: Vec<Vec<i32>> = vec![];
    let len = b.len();

    //This is always the first point
    points.push(vec![b[0][0], b[0][2]]);

    for i in 1..len {
        let mut last_overlap = 0;

        'inner: for j in (i..len).rev() {
            if b[j][0] <= b[i - 1][1] || b[j][1] < b[i - 1][1] {
                if b[j][1] < b[i - 1][1] {
                    last_overlap = b[j][1];
                } else {
                    last_overlap = b[i - 1][1];
                }
                //Check if the current segment we are looking at is intersected,
                // if so, push the leftmost point but make sure not to double push
                //this is likely a good candidate for memoization-based improvement
                if (b[i - 1][1] + b[i - 1][0]) > b[j][0] && b[i - 1][2] > b[j][2] {
                    let point = vec![b[i - 1][0], b[i - 1][2]];
                    let next_point = vec![b[i - 1][1], b[j][2]];

                    //prevent duplicate pushes
                    if points[points.len() - 1] != point {
                        points.push(point);
                    }
                    if points.len() > 2 {
                        //prevent duplicate pushes
                        if points[points.len() - 2] != next_point
                        {
                            points.push(next_point);
                        }
                    } else {
                        points.push(next_point);
                    }
                }
            } else if last_overlap > b[i - 1][1] {
                break 'inner;
            }
        }

        if last_overlap == 0 {
            points.push(vec![b[i - 1][1], 0]);
            points.push(vec![b[i][0], b[i][2]]);
        }
    }

    //This is always the last point
    points.push(vec![b[len - 1][1], 0]);

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_building_overlap() {
        let buildings: Vec<Vec<i32>> = vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ];

        let sol: Vec<Vec<i32>> = vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0],
        ];

        assert_eq!(get_skyline(buildings), sol);
    }

    #[test]
    fn edge_case_no_horizontal_segment_intersect() {
        let buildings: Vec<Vec<i32>> = vec![vec![0, 2, 3], vec![2, 5, 3]];

        let sol: Vec<Vec<i32>> = vec![vec![0, 3], vec![5, 0]];

        assert_eq!(get_skyline(buildings), sol);
    }

    /// check multiple series of multi-building (more than 2) overlap and no overlap
    #[test]
    fn edge_case_multiple_terminates() {
        let buildings: Vec<Vec<i32>> = vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
            vec![26, 27, 9],
        ];
        let sol: Vec<Vec<i32>> = vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0],
            vec![26, 9],
            vec![27, 0],
        ];

        assert_eq!(get_skyline(buildings), sol);
    }

    /// Check no overlap
    #[test]
    fn edge_case_no_horizontal_segment_intersect_no_overlap() {
        let buildings: Vec<Vec<i32>> = vec![vec![0, 2, 3], vec![3, 6, 3]];

        let sol: Vec<Vec<i32>> = vec![vec![0, 3], vec![2, 0], vec![3, 3], vec![6, 0]];

        assert_eq!(get_skyline(buildings), sol);
    }

    /* Not sure this case actually meets problem spec-- the buildings that show up
    in the Vec<Vec<i32>> would only be buildings that contribute to at least one
    skyline point? Can we make that assumption? If not we need to add in a bunch of
    checks to see if we are pushing a point that is subsumed by an existing point
    /// Check that nested buildings dont have points
    #[test]
    fn edge_case_nested_no_corners() {
        let buildings: Vec<Vec<i32>> = vec![vec![1, 10, 10], vec![2, 3, 3], vec![5, 8, 6]];

        let sol: Vec<Vec<i32>> = vec![vec![1, 10], vec![10, 0]];

        assert_eq!(get_skyline(buildings), sol);
    }*/
}
