use crate::geometry::{sort_points_by_x, Edge, Point, Triangle, Vec3D};
use std::cmp::Ordering;

// https://dccg.upc.edu/people/vera/wp-content/uploads/2014/11/GA2014-ConvexHulls3D-Roger-Hernando.pdf

pub fn run(mut data: Vec<Point>) -> Vec<Triangle> {
    sort_points_by_xyz(&mut data);
    run_sorted(data)
}

fn sort_points_by_xyz(data: &mut Vec<Point>) {
    data.sort_by(|pt1, pt2| {
        if pt1.x < pt2.x {
            return Ordering::Less;
        } else if pt1.x > pt2.x {
            return Ordering::Greater;
        }
        if pt1.y < pt2.y {
            return Ordering::Less;
        } else if pt1.y > pt2.y {
            return Ordering::Greater;
        }
        if pt1.z < pt2.z {
            return Ordering::Less;
        } else if pt1.z > pt2.z {
            return Ordering::Greater;
        }
        Ordering::Equal
    });
}

pub fn run_sorted(mut data: Vec<Point>) -> Vec<Triangle> {
    let mut result: Vec<Triangle> = vec![];
    let mut queue: Vec<Edge> = vec![first_edge(&mut data)];
    while !queue.is_empty() && !data.is_empty() {
        let edge = queue.pop().unwrap();
        let p3 = match find_triangle_third_vertex(&mut data, edge) {
            Some(val) => val,
            None => {
                if result.is_empty() {
                    panic!("1st edge is bad")
                }
                eprintln!(
                    "result.len(): {}\tremained_data.len(): {}",
                    result.len(),
                    data.len()
                );
                panic!("no 3rd point selected")
            }
        };
        queue.push(Edge::new(edge.p1.clone(), p3.clone()));
        queue.push(Edge::new(edge.p2.clone(), p3.clone()));
        result.push(Triangle::new(edge.p1.clone(), edge.p2.clone(), p3));
        // progress_indicator(&result, &data);
    }
    result
}

fn progress_indicator(result: &Vec<Triangle>, remained_data: &Vec<Point>) {
    if result.len() % 2 == 0 {
        println!(
            "result.len(): {}\tremained_data.len(): {}",
            result.len(),
            remained_data.len()
        );
    }
}

// fn first_edge(data: &mut Vec<Point>) -> Edge {
//     let p1 = data.pop().unwrap();
//     let p2 = data.pop().unwrap();
//     Edge::new(p1, p2)
// }

fn first_edge(data: &mut Vec<Point>) -> Edge {
    let mut index1: Option<usize> = None;
    let mut index2: Option<usize> = None;
    let mut selected_p2: Option<Point> = None;
    for (i, p1) in data.iter().enumerate() {
        for (j, p2) in data[i + 1..].iter().enumerate() {
            let j = j + i + 1;
            match find_triangle_third_vertex_index(data, Edge::new(p1.clone(), p2.clone())) {
                None => continue,
                Some(_) => {
                    index1 = Some(i);
                    if j > i {
                        index2 = Some(j - 1);
                    } else {
                        index2 = Some(j);
                    }
                    selected_p2 = Some(p2.clone());
                    break;
                }
            };
        }
    }
    let p1 = data.remove(index1.unwrap());
    let p2 = data.remove(index2.unwrap());
    if p2 != selected_p2.unwrap() {
        panic!("bug, p2 is not the same as selected one")
    }
    Edge::new(p1, p2)
}

// pick a point to form triangle with the edge.
// the triangle will have all other points on one side of the plane it is on.
fn find_triangle_third_vertex(data: &mut Vec<Point>, edge: Edge) -> Option<Point> {
    match find_triangle_third_vertex_index(data, edge) {
        Some(i) => {
            let p3 = data.remove(i);
            Some(p3)
        }
        None => None, // panic!("no third point found to form a good triangle")
    }
}

fn find_triangle_third_vertex_index(data: &Vec<Point>, edge: Edge) -> Option<usize> {
    let mut index: Option<usize> = None;
    for (i, pt) in data.iter().enumerate() {
        if pt == &edge.p1 || pt == &edge.p2 {
            continue;
        }
        let tri = Triangle::new(edge.p1.clone(), edge.p2.clone(), pt.clone());
        if all_pts_on_inside(&data, &tri) {
            index = Some(i);
            break;
        }
        // swap p1 and p2
        let tri = Triangle::new(edge.p1.clone(), pt.clone(), edge.p2.clone());
        if all_pts_on_inside(&data, &tri) {
            index = Some(i);
            break;
        }
    }
    index
}

fn all_pts_on_inside(data: &[Point], tri: &Triangle) -> bool {
    for pt in data {
        if pt == &tri.p1 || pt == &tri.p2 || pt == &tri.p3 {
            // skip identical points as in the triangle
            continue;
        }
        let (normal_vec, normal_pt) = triangle_normal(&tri);
        let vec = Vec3D::new(pt.x - normal_pt.x, pt.y - normal_pt.y, pt.z - normal_pt.z);
        let product = dot_product(&normal_vec, &vec);
        if product == 0.0 {
            // eprintln!(
            //     "({}, {}, {}) and {} are coplanar",
            //     tri.p1, tri.p2, tri.p3, pt
            // );

            // treat coplanar case as inside
        }
        // denote negative dot product as outside and positive as inside.
        if product < 0.0 {
            return false;
        }
    }
    true
}

fn triangle_normal(tri: &Triangle) -> (Vec3D, Point) {
    let a = Vec3D::new(
        tri.p2.x - tri.p1.x,
        tri.p2.y - tri.p1.y,
        tri.p2.z - tri.p1.z,
    );
    let b = Vec3D::new(
        tri.p3.x - tri.p1.x,
        tri.p3.y - tri.p1.y,
        tri.p3.z - tri.p1.z,
    );
    // p1->p2 X p1->p3
    (cross_product(&a, &b), tri.p1)
}

fn cross_product(a: &Vec3D, b: &Vec3D) -> Vec3D {
    Vec3D {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

fn dot_product(a: &Vec3D, b: &Vec3D) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

#[cfg(test)]
mod tests {
    use super::run as jarvis_march;
    use crate::geometry::{print_pts, sort_points_by_x, Point};
    use rand::prelude::ThreadRng;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_jarvis_march_3d() {
        let test_data = vec![
            Point::new(0.7851142197611858, 0.7560614242016357, 0.44320903675329393),
            Point::new(0.7938878582088675, 0.15596477112008822, 0.06618033991950223),
            Point::new(0.5199616108281361, 0.828528810211144, 0.30987530057703594),
            Point::new(0.7346805422251943, 0.9584402336037428, 0.41035012146216565),
            Point::new(0.6685642915866097, 0.4871370877098564, 0.5317390582569287),
            Point::new(0.24679526601168777, 0.6582204171802175, 0.06580728835450844),
            Point::new(0.9174155290104739, 0.1873024003404007, 0.7647056916428465),
            Point::new(0.7177134263025706, 0.5641305651337418, 0.3457068575258073),
            Point::new(0.5234468835170234, 0.9422283433025127, 0.8341827694942268),
            Point::new(0.7662765708922703, 0.571802514850265, 0.30569811331872554),
        ];
        let result = jarvis_march(test_data);
        for tri in result {
            println!("{}, {}, {}", tri.p1, tri.p2, tri.p3);
        }
    }

    #[test]
    fn test_jarvis_march_3d_rand_10() {
        for _ in 0..100 {
            let mut rng: ThreadRng = thread_rng();
            let mut test_data = generate_test_data(10);
            print_pts(&test_data);

            let result = jarvis_march(test_data);
            for tri in result {
                println!("{}, {}, {}", tri.p1, tri.p2, tri.p3);
            }
            println!("---\n\n");
        }
    }

    #[test]
    fn test_jarvis_march_3d_rand_100() {
        let mut rng: ThreadRng = thread_rng();
        let mut test_data = generate_test_data(100);
        print_pts(&test_data);

        let result = jarvis_march(test_data);
        for tri in result {
            println!("{}, {}, {}", tri.p1, tri.p2, tri.p3);
        }
    }

    fn generate_test_data(count: usize) -> Vec<Point> {
        let mut rng: ThreadRng = thread_rng();
        let mut test_data = vec![];
        while test_data.len() < count {
            let pt = generate_pt(&mut rng);
            if !test_data.contains(&pt) {
                test_data.push(pt);
            }
        }
        test_data
    }

    fn generate_pt(rng: &mut ThreadRng) -> Point {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        Point::new(x, y, z)
    }
}
