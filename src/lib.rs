extern crate float_eq;
extern crate robust_orientation;

use std::cmp::Ordering;
use float_eq::feq;
use robust_orientation::orientation_2d;

//Computes the convex hull of a set of 2D points.
//Input   : an sequence of (x, y) pairs representing the input points.
//Output  : a of vertices of the convex hull in counter-clockwise order,
//  starting from the vertex with the lexicographically smallest coordinates.
//Ref: Andrew's monotone chain algorithm. O(n log n) complexity.
fn convex_hull_2d(coordinates: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut upper: Vec<Vec<f64>> = Vec::new();
    let mut lower: Vec<Vec<f64>> = Vec::new();
    let mut coords: Vec<Vec<f64>> = Vec::with_capacity(coordinates.len());
    coords.extend(coordinates.iter().cloned());

    let n = coords.len();
    if n < 3 {
        if n == 2 &&
            coords[0][0] == coords[1][0] &&
            coords[0][1] == coords[1][1] {
            return vec!(coords[0].clone());
        }
        return coords;
    }

    sort_2d(&mut coords);

    for pt in coords.iter() {
        // should go clockwise
        // if counter or on line pop
        while upper.len() > 1 &&
            orientation_2d(
                &upper[upper.len() - 2],
                &upper[upper.len() - 1],
                &pt) <= 0f64 {
            upper.pop();
        }

        // should go counter clock
        // if clockwise or on line pop
        while lower.len() > 1 &&
            orientation_2d(
                &lower[lower.len() - 2],
                &lower[lower.len() - 1],
                &pt) >= 0f64 {
            lower.pop();
        }

        upper.push(pt.clone());
        lower.push(pt.clone());
    }

    // or upper = [o for o in upper]
    upper.reverse();

    //end points are repeated top hull & down hull
    // return lower[:-1] + upper[:-1], lower, or upper
    if lower.len() > 0 {
        lower = lower[0..lower.len() - 1].to_vec()
    }
    if upper.len() > 0 {
        upper = upper[0..upper.len() - 1].to_vec()
    }

    for pt in upper.iter() {
        lower.push(pt.clone())
    }
    return upper;
}


fn sort_2d(coords: &mut [Vec<f64>]) {
    coords.sort_by(|a, b| lexsort_2d(a, b));
}

//sort 2d coordinates lexicographically
fn lexsort_2d(a: &[f64], b: &[f64]) -> std::cmp::Ordering {
    let mut d = a[0] - b[0];
    if feq(d, 0.0) {
        d = a[1] - b[1];
    }

    if feq(d, 0.0) {
        return Ordering::Equal;
    } else if d < 0.0 {
        return Ordering::Less;
    }
    return Ordering::Greater;
}


#[cfg(test)]
mod test_convexes {
    use super::convex_hull_2d;

    #[test]
    fn test_convex_hull() {
        let xy = vec!(vec!(0., 0.), vec!(1., 1.), vec!(1., 0.), vec!(0.5, 0.5), vec!(0.7, 0.1));
        let hull = convex_hull_2d(&xy);
        println!("{:?}", hull);
    }
}
