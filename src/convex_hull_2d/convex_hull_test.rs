#[cfg(test)]
mod tests {
    use super::super::convex_hull::{andrew_algo, jarvis_march};
    use crate::convex_hull_2d::convex_hull::andrew_algo_sort;
    use crate::geometry::{sort_points_by_x, Point};

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
    fn test_jarvis_march() {
        let all_test_cases = all_convex_hull_test_cases();

        for (i, test_case) in all_test_cases.iter().enumerate() {
            let hull = jarvis_march(&test_case.data);
            println!("test_case {}", i);
            assert_eq!(hull.len(), test_case.expected_hull.len());
            for pt1 in &hull {
                assert!(test_case.expected_hull.contains(pt1));
            }
        }
    }
    #[test]
    fn test_andrew_scan() {
        let all_test_cases = all_convex_hull_test_cases();

        for (i, test_case) in all_test_cases.iter().enumerate() {
            let mut test_data = test_case.data.clone();
            let hull = andrew_algo_sort(&mut test_data);
            assert_eq!(hull.len(), test_case.expected_hull.len());
            for pt1 in &hull {
                assert!(test_case.expected_hull.contains(pt1));
            }
        }
    }

    fn print_pt_list(data: &[Point]) {
        for pt in data {
            println!("{}", pt);
        }
    }
}
