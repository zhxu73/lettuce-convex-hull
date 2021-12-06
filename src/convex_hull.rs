use crate::geometry::{sort_points_by_x, Point};

pub fn test_data() -> Vec<Point> {
    vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(5.0, 0.0, 0.0),
        Point::new(0.0, 5.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ]
}

pub fn convex_hull(mut data: Vec<Point>) -> Vec<Point> {
    // sort by x
    sort_points_by_x(&mut data);

    let mut result = chan_algo(&data);
    for pt in &result {
        println!("{}", pt);
    }
    sort_points_by_x(&mut result);
    result
}

fn chan_algo(data: &[Point]) -> Vec<Point> {
    const sub_hull_count: usize = 1200;
    println!("per hull: {}", data.len() / sub_hull_count);
    let mut all_sub_hull: Vec<Point> = vec![];

    let mut pt_counter = 0;

    for i in 0..sub_hull_count {
        let start = i * data.len() / sub_hull_count;
        let end = match (i + 1) * data.len() / sub_hull_count < data.len() {
            true => (i + 1) * data.len() / sub_hull_count,
            false => data.len(),
        };
        pt_counter += data[start..end].len();
        let sub_hull = andrew_algo(&data[start..end]);
        all_sub_hull.extend(sub_hull);
    }
    if data.len() != pt_counter {
        eprintln!("{} != {}", data.len(), pt_counter);
        panic!("point count does not match");
    }
    let result = javis_march(all_sub_hull.as_slice());
    javis_march(result.as_slice())
}

fn andrew_algo(data: &[Point]) -> Vec<Point> {
    let mut result: Vec<Point> = vec![data.get(0).unwrap().clone(), data.get(1).unwrap().clone()];
    // upper hull
    for pt in (data[2..]).iter() {
        while result.len() >= 2 {
            let last = result.last().unwrap();
            let second_last = result.get(result.len() - 2).unwrap();
            if orientation(second_last, last, pt) <= 0 {
                result.pop();
            } else {
                break;
            }
        }
        result.push(pt.clone());
    }
    let upper_hull_len = result.len();
    // println!("upper hull len: {}", upper_hull_len);

    // lower hull
    for pt in data.iter().rev() {
        while result.len() >= upper_hull_len {
            let last = result.last().unwrap();
            let second_last = result.get(result.len() - 2).unwrap();
            if orientation(second_last, last, pt) <= 0 {
                result.pop();
            } else {
                break;
            }
        }
        result.push(pt.clone());
    }
    result
}

fn javis_march(data: &[Point]) -> Vec<Point> {
    let pt_1st = match data.get(0) {
        None => {
            panic!("no points")
        }
        Some(val) => val,
    };

    let mut result: Vec<Point> = vec![*pt_1st];
    for _ in 0..data.len() {
        let selected = match select_pt(data, result.last().unwrap()) {
            None => {
                // no points selected
                return result;
            }
            Some(val) => val,
        };
        if &selected == pt_1st {
            break;
        }
        result.push(selected);
    }

    result
}

fn select_pt(data: &[Point], last_hull_pt: &Point) -> Option<Point> {
    let pt1 = last_hull_pt;
    for pt2 in data {
        if pt2 == last_hull_pt {
            continue;
        }

        // eprintln!("pt1: {}", pt1);
        // eprintln!("pt2: {}", pt2);

        let mut ok = true;
        // selected pt2 must have all other points on the right of pt1->pt2
        for pt3 in data {
            if pt3 == last_hull_pt {
                continue;
            }
            if pt3 == pt2 {
                continue;
            }
            let ori = orientation(pt1, pt2, pt3);
            // eprintln!("pt3 {}, ori {}", pt3, ori);
            if ori <= 0 {
                ok = false;
                break;
            }
        }
        if ok {
            return Some((*pt2).clone());
        }
    }
    None
}

// pt1->pt2 X pt1->pt3
fn orientation(pt1: &Point, pt2: &Point, pt3: &Point) -> i32 {
    // ax * by - ay * bx
    let z = (pt2.x - pt1.x) * (pt3.y - pt1.y) - (pt2.y - pt1.y) * (pt3.x - pt1.x);
    let result = if z == 0.0 {
        0
    } else if z > 0.0 {
        1
    } else {
        -1
    };
    result
}

#[cfg(test)]
mod tests {
    use crate::convex_hull::{andrew_algo, javis_march};
    use crate::geometry::Point;

    struct convex_hull_test_case {
        data: Vec<Point>,
        expected_hull: Vec<Point>,
    }
    fn all_convex_hull_test_cases() -> Vec<convex_hull_test_case> {
        vec![
            convex_hull_test_case {
                data: vec![
                    Point::new2d_int(262, -103),
                    Point::new2d_int(93, 120),
                    Point::new2d_int(162, 52),
                    Point::new2d_int(126, -167),
                    Point::new2d_int(-95, 140),
                    Point::new2d_int(-78, -114),
                ],
                expected_hull: vec![
                    Point::new2d_int(-95, 140),
                    Point::new2d_int(93, 120),
                    Point::new2d_int(162, 52),
                    Point::new2d_int(262, -103),
                    Point::new2d_int(126, -167),
                    Point::new2d_int(-78, -114),
                ],
            },
            convex_hull_test_case {
                data: vec![
                    Point::new2d_int(-279, -115),
                    Point::new2d_int(-247, -41),
                    Point::new2d_int(-158, -6),
                    Point::new2d_int(-153, -68),
                    Point::new2d_int(23, -4),
                    Point::new2d_int(235, -130),
                ],
                expected_hull: vec![
                    Point::new2d_int(-279, -115),
                    Point::new2d_int(-247, -41),
                    Point::new2d_int(-158, -6),
                    Point::new2d_int(23, -4),
                    Point::new2d_int(235, -130),
                ],
            },
            convex_hull_test_case {
                data: vec![
                    Point::new2d_int(-270, 45),
                    Point::new2d_int(-43, -38),
                    Point::new2d_int(-10, 13),
                    Point::new2d_int(126, -140),
                    Point::new2d_int(156, 77),
                    Point::new2d_int(181, -48),
                ],
                expected_hull: vec![
                    Point::new2d_int(-270, 45),
                    Point::new2d_int(156, 77),
                    Point::new2d_int(181, -48),
                    Point::new2d_int(126, -140),
                ],
            },
            convex_hull_test_case {
                data: vec![
                    Point::new2d_int(-202, 85),
                    Point::new2d_int(-99, 109),
                    Point::new2d_int(-97, -116),
                    Point::new2d_int(-54, 1),
                    Point::new2d_int(34, 31),
                    Point::new2d_int(40, -94),
                    Point::new2d_int(77, 168),
                    Point::new2d_int(81, 115),
                    Point::new2d_int(175, 23),
                    Point::new2d_int(232, 53),
                    Point::new2d_int(237, -161),
                    Point::new2d_int(243, -152),
                ],
                expected_hull: vec![
                    Point::new2d_int(-202, 85),
                    Point::new2d_int(77, 168),
                    Point::new2d_int(232, 53),
                    Point::new2d_int(243, -152),
                    Point::new2d_int(237, -161),
                    Point::new2d_int(-97, -116),
                ],
            },
            convex_hull_test_case {
                data: vec![
                    Point::new2d_int(-21, -124),
                    Point::new2d_int(272, 68),
                    Point::new2d_int(-221, -174),
                    Point::new2d_int(-212, 126),
                    Point::new2d_int(-151, -23),
                    Point::new2d_int(-58, 76),
                    Point::new2d_int(-53, -81),
                    Point::new2d_int(130, 114),
                    Point::new2d_int(245, -24),
                    Point::new2d_int(85, -40),
                ],
                expected_hull: vec![
                    Point::new2d_int(-221, -174),
                    Point::new2d_int(-212, 126),
                    Point::new2d_int(130, 114),
                    Point::new2d_int(272, 68),
                    Point::new2d_int(245, -24),
                    Point::new2d_int(-21, -124),
                ],
            },
        ]
    }

    #[test]
    fn test_javis_march() {
        let all_test_cases = all_convex_hull_test_cases();

        for (i, test_case) in all_test_cases.iter().enumerate() {
            let hull = javis_march(&test_case.data);
            println!("test_case {}", i);
            assert_eq!(hull.len(), test_case.expected_hull.len());
            for pt1 in &hull {
                let mut found = false;
                for pt2 in &test_case.expected_hull {
                    if pt1 == pt2 {
                        found = true;
                        break;
                    };
                }
                assert!(found);
            }
        }
    }
    #[test]
    fn test_andrew_scan() {
        let all_test_cases = all_convex_hull_test_cases();

        for (i, test_case) in all_test_cases.iter().enumerate() {
            let hull = andrew_algo(&test_case.data);
            println!("test_case {}", i);
            assert_eq!(hull.len(), test_case.expected_hull.len());
            for pt1 in &hull {
                let mut found = false;
                for pt2 in &test_case.expected_hull {
                    if pt1 == pt2 {
                        found = true;
                        break;
                    };
                }
                assert!(found);
            }
        }
    }
}
