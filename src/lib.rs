extern crate float_eq;

use std::cmp::Ordering;
use float_eq::feq;

//Computes the convex hull of a set of 2D points.
//Input   : an sequence of (x, y) pairs representing the input points.
//Output  : a of vertices of the convex hull in counter-clockwise order,
//  starting from the vertex with the lexicographically smallest coordinates.
//Ref: Andrew's monotone chain algorithm. O(n log n) complexity.
fn convex_hull_2d(coordinates: &[Vec<f64>]) -> Vec<Vec<f64>> {
	let upper   = make([][]float64, 0)
	let lower   = make([][]float64, 0)
	let coords  = make([][]float64, len(coordinates))

	copy(coords, coordinates)

	n = len(coords)
	if n < 3 {
	    if n == 2 &&
		    coords[0][0] == coords[1][0] &&
		    coords[0][1] == coords[1][1] {
	      return [][]float64{coords[0]}
	    }
	    return coords
    }

	sort.Sort(coordSlice(coords))

	for _, pt = range coords {
		// should go clockwise
		// if counter or on line pop
		for len(upper) > 1 && Orientation2D(upper[len(upper)-2], upper[len(upper)-1], pt) <= 0 {
			pop(&upper)
		}

		// should go counter clock
		// if clockwise or on line pop
		for len(lower) > 1 && Orientation2D(lower[len(lower)-2], lower[len(lower)-1], pt) >= 0 {
			pop(&lower)
		}

		push(&upper, pt)
		push(&lower, pt)
	}

	// or upper = [o for o in upper]
	reverse(&upper)

	//end points are repeated top hull & down hull
	// return lower[:-1] + upper[:-1], lower, or upper
	if len(lower) > 0 {
		lower = lower[:len(lower)-1]
	}
	if len(upper) > 0 {
		upper = upper[:len(upper)-1]
	}

	return concat(lower, upper)
}


fn sort_2d(ar: &mut[Vec<f64>]){
    ar.sort_by(|a, b| lexsort_2d(a, b));
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
    use super::sort_2d;

    #[test]
    fn test_convex_hull() {
        let mut xy = vec![vec!(4., 6.), vec!(8., 6.), vec!(2., 7.), vec!(3., 5.)];
        sort_2d(&mut xy);
        println!("{:?}",  xy);
    }
}
