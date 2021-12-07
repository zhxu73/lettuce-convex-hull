use super::jarvis_march;
use crate::geometry::{sort_points_by_x, triangles_to_pts, Edge, Point, Triangle, Vec3D};

pub fn run(mut data: Vec<Point>) -> Vec<Triangle> {
    sort_points_by_x(&mut data);
    let mut sub_hull_count: usize = 1000;
    if data.len() < 1000 {
        sub_hull_count = 1;
    } else if data.len() < 10000 {
        sub_hull_count = 100;
    }
    println!("per hull: {}", data.len() / sub_hull_count);
    let mut all_sub_hull_result: Vec<Triangle> = vec![];
    // for (index, chunk) in data.chunks(sub_hull_count).enumerate() {
    //     let mut data_slice: Vec<Point> = vec![];
    //     data_slice.extend_from_slice(&chunk);
    //     let result = jarvis_march::run(data_slice);
    //     all_sub_hull_result.extend(result);
    // }
    for index in 0..sub_hull_count {
        let (start, end) = chan_sub_hull_range(data.len(), index, sub_hull_count);
        let mut data_slice: Vec<Point> = vec![];
        data_slice.extend_from_slice(&data[start..end]);
        let result = jarvis_march::run(data_slice);
        all_sub_hull_result.extend(result);
    }
    jarvis_march::run(triangles_to_pts(all_sub_hull_result))
}

fn chan_sub_hull_range(
    data_len: usize,
    sub_hull_index: usize,
    sub_hull_count: usize,
) -> (usize, usize) {
    let start = sub_hull_index * data_len / sub_hull_count;
    let end = match (sub_hull_index + 1) * data_len / sub_hull_count < data_len {
        true => (sub_hull_index + 1) * data_len / sub_hull_count,
        false => data_len,
    };
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::run as chan;
    use crate::geometry::{print_pts, sort_points_by_x, Point};
    use rand::prelude::ThreadRng;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_chan_3d_1() {
        let test_data = vec![
            Point::new(0.3378229813360598, 0.1289199928685838, 0.6349226539880101),
            Point::new(0.2585937589239671, 0.2935847684867271, 0.9847373158977359),
            Point::new(0.971658940526966, 0.8406186443800302, 0.8828580831016223),
            Point::new(0.7894023696999499, 0.2692046467493203, 0.2184541641381902),
            Point::new(0.1919364757323635, 0.2535306160339036, 0.12560501844534644),
            Point::new(0.08886851960828668, 0.36749853287024736, 0.9804408401884549),
            Point::new(
                0.24119743925942538,
                0.15616573452062465,
                0.37515188977887526,
            ),
            Point::new(0.5738493744204595, 0.8741496442538784, 0.6205864740099618),
            Point::new(
                0.22185028524446482,
                0.21686495306542297,
                0.25269194440896325,
            ),
            Point::new(0.23000306733390696, 0.07355483001280383, 0.5170732459676636),
        ];

        let result = chan(test_data);
        for tri in result {
            println!("{}, {}, {}", tri.p1, tri.p2, tri.p3);
        }
    }

    #[test]
    fn test_chan_3d_2() {
        let test_data = vec![
            Point::new(0.4655582122316626, 0.891910438079256, 0.5828807159230491),
            Point::new(0.3444799652724141, 0.8048785795819392, 0.9393659130800363),
            Point::new(0.8136883814410312, 0.04220012035787102, 0.3088311928181018),
            Point::new(0.41134092590085847, 0.976495226226626, 0.7282815671147338),
            Point::new(0.4225573842735715, 0.4153662628361967, 0.9326798448789545),
            Point::new(0.09963220760330005, 0.7242778129588525, 0.9160706778590151),
            Point::new(0.21921228036689333, 0.729241878091233, 0.9682048502235978),
            Point::new(
                0.6441136596209192,
                0.6322774356010346,
                0.0010892873587931007,
            ),
            Point::new(0.61844025354729, 0.4858829864069433, 0.7151249819093812),
            Point::new(0.1904018057684721, 0.3093473600982406, 0.7978016509510075),
        ];

        let result = chan(test_data);
        for tri in result {
            println!("{}, {}, {}", tri.p1, tri.p2, tri.p3);
        }
    }

    #[test]
    fn test_chan_3d_rand_10() {
        for _ in 0..10 {
            let mut rng: ThreadRng = thread_rng();
            let mut test_data = generate_test_data(10);
            print_pts(&test_data);

            let result = chan(test_data);
            for tri in result {
                println!("{}, {}, {}", tri.p1, tri.p2, tri.p3);
            }
            println!("-----\n\n")
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
