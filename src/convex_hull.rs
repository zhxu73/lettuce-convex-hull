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
    let result = javis_march(&data);
    for pt in &result {
        println!("{}", pt);
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
