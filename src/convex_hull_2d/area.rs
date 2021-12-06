use super::convex_hull::jarvis_march;
use crate::geometry::Point;

#[allow(dead_code)]
// reorder the points before compute the 2D area (x-y plane).
pub fn convex_hull_area_reorder(hull: &[Point]) -> f64 {
    let reordered = jarvis_march(hull);
    convex_hull_area(&reordered)
}

#[allow(dead_code)]
// compute the 2D area (x-y plane).
// Note: this assumes the vertices are ordered (clockwise or counter-clockwise).
pub fn convex_hull_area(hull: &[Point]) -> f64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    // https://www.mathwords.com/a/area_convex_polygon.htm
    let mut first_sum = 0.0f64;
    for i in 0..hull.len() {
        if i + 1 < hull.len() {
            first_sum += hull[i].x * hull[i + 1].y;
        } else {
            // wrap around on last
            first_sum += hull[i].x * hull[0].y;
        }
        // println!("1st: {}", first_sum);
    }
    let mut second_sum = 0.0f64;
    for i in 0..hull.len() {
        if i + 1 < hull.len() {
            second_sum += hull[i].y * hull[i + 1].x;
        } else {
            // wrap around on last
            second_sum += hull[i].y * hull[0].x;
        }
        // println!("2nd: {}", second_sum);
    }
    // take absolute value to account for possible clockwise order.
    ((first_sum - second_sum) / 2.0).abs()
}

#[cfg(test)]
mod tests {
    use super::{convex_hull_area, convex_hull_area_reorder};
    use crate::geometry::Point;

    struct test_data {
        data: Vec<Point>,
        expected_area: f64,
        error: bool,
    }

    #[test]
    fn test_convex_hull_area() {
        let all_data: Vec<test_data> = vec![
            test_data {
                data: vec![
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(0.0, 5.0, 0.0),
                    Point::new(3.0, 0.0, 0.0),
                ],
                expected_area: (3.0 * 5.0) / 2.0,
                error: false,
            },
            test_data {
                data: vec![
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(6.6, 0.0, 0.0),
                    Point::new(3.3, 1.3, 0.0),
                ],
                expected_area: (6.6 * 1.3) / 2.0,
                error: false,
            },
            test_data {
                data: vec![
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(6.6, 0.0, 0.0),
                    Point::new(6.6, 3.3, 0.0),
                    Point::new(0.0, 3.3, 0.0),
                ],
                expected_area: 6.6 * 3.3,
                error: false,
            },
            test_data {
                data: vec![
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(6.6, 0.0, 0.0),
                    Point::new(0.0, 3.3, 0.0), // swapped compare to last test case
                    Point::new(6.6, 3.3, 0.0), // swapped compare to last test case
                ],
                expected_area: 0.0,
                error: true, // previous test case, but messed up the order of points
            },
        ];

        for case in all_data {
            let area = convex_hull_area(&case.data);
            let tolerance = 0.000001;
            if !case.error && (area - case.expected_area).abs() > tolerance {
                eprintln!("area: {}, expected: {}", area, case.expected_area);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_convex_hull_area_reorder() {
        let all_data: Vec<test_data> = vec![
            test_data {
                data: vec![
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(6.6, 0.0, 0.0),
                    Point::new(6.6, 3.3, 0.0),
                    Point::new(0.0, 3.3, 0.0),
                ],
                expected_area: 6.6 * 3.3,
                error: false,
            },
            test_data {
                data: vec![
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(6.6, 0.0, 0.0),
                    Point::new(0.0, 3.3, 0.0), // swapped compare to last test case
                    Point::new(6.6, 3.3, 0.0), // swapped compare to last test case
                ],
                expected_area: 6.6 * 3.3,
                error: false,
            },
        ];
        for case in all_data {
            let area = convex_hull_area_reorder(&case.data);
            let tolerance = 0.000001;
            if !case.error && (area - case.expected_area).abs() > tolerance {
                eprintln!("area: {}, expected: {}", area, case.expected_area);
                assert!(false);
            }
        }
    }
}
