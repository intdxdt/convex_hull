extern crate float_eq;

use std::cmp::Ordering;
use float_eq::feq;

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
