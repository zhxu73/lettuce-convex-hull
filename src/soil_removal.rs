use crate::geometry::{sort_points_by_x, Point};

#[allow(dead_code)]
pub fn remove_soil_simple(mut points: Vec<Point>) -> Vec<Point> {
    let mut min_z: f64 = f64::MAX;
    let mut max_z: f64 = 0.0;
    for pt in &points {
        if pt.z < min_z {
            min_z = pt.z
        }
        if pt.z > max_z {
            max_z = pt.z
        }
    }
    // sort_by x
    sort_points_by_x(&mut points);
    let bucket_count = 100;
    let bucket_width = (max_z - min_z) / (bucket_count as f64);
    let mut result = vec![];
    for pt in points {
        let diff = pt.z - min_z;
        let mut bucket_index = (diff / bucket_width) as usize;
        if bucket_index >= bucket_count {
            bucket_index = bucket_count;
        }
        if bucket_index > 43 {
            result.push(pt);
        }
    }
    result
}

#[allow(dead_code)]
pub fn remove_soil(mut points: Vec<Point>) -> Vec<Point> {
    // min_y, max_y
    let mut min_y: f64 = f64::MAX;
    let mut max_y: f64 = 0.0;
    for pt in &points {
        if pt.y < min_y {
            min_y = pt.y
        }
        if pt.y > max_y {
            max_y = pt.y
        }
    }
    // sort_by x
    sort_points_by_x(&mut points);

    let band_count = 2000;
    let mut all_bands: Vec<Band> = vec![];
    all_bands.reserve(band_count);
    for i in 0..band_count {
        all_bands.push(Band {
            index: i,
            last_pt: None,
            previous_pts: vec![],
            first_boundary_detected: false,
            in_boundary: false,
        });
    }
    let band_width: f64 = (max_y - min_y) / band_count as f64;

    let mut result: Vec<Point> = vec![];
    result.reserve(band_count);
    let mut all_d: Vec<f64> = vec![];

    for pt in &points {
        // check which band the pt fall into
        let band_index = ((pt.y - min_y) / band_width) as usize;
        let band = all_bands.get_mut(band_index).unwrap();

        process_band(band, pt, &mut result);

        match band.last_pt {
            None => {
                band.last_pt = Some(pt.clone());
                continue;
            }
            Some(_) => {}
        };

        let d = band.derivative(pt);
        if f64::is_infinite(d) {
            continue;
        }
        all_d.push(d.clone());
        if d.abs() > 80000.0 {
            println!(
                "d: #{}, {}, {}, {}",
                band_index,
                d,
                pt.x - band.last_pt.unwrap().x,
                pt.z - band.last_pt.unwrap().z
            );
            result.push(band.last_pt.unwrap().clone());
            result.push(pt.clone());
        }

        band.boundary_detect(pt);
        if band.in_boundary {
            // result.push(band.last_pt.unwrap().clone());
            // result.push(pt.clone());
        }
        band.last_pt = Some(pt.clone());
    }
    // all_d.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut infi_count = 0;
    for d1 in all_d {
        // print!("{}, ", d1);
        if f64::is_infinite(d1) {
            infi_count += 1;
        }
    }
    println!("infi count: {}", infi_count);
    result
}

struct Band {
    pub index: usize,
    pub last_pt: Option<Point>,
    pub previous_pts: Vec<Point>,
    pub first_boundary_detected: bool,
    pub in_boundary: bool,
}
impl Band {
    fn derivative(&self, new_pt: &Point) -> f64 {
        derivative(&self.last_pt.unwrap(), new_pt)
    }
    fn boundary_detect(&mut self, new_pt: &Point) {
        let d = self.derivative(new_pt);
        if !self.first_boundary_detected {
            if d > 3000.0 {
                self.first_boundary_detected = true;
                self.in_boundary = true;
            }
        } else {
            if d < -3000.0 {
                self.in_boundary = false;
            }
        }
    }
    fn add_pt(&mut self, new_pt: &Point) {
        self.previous_pts.push(new_pt.clone());
        if self.previous_pts.len() > 5 {
            self.previous_pts.remove(0);
        }
    }

    fn previous_pts_derivative(&self) -> f64 {
        if self.previous_pts.len() <= 1 {
            panic!();
        }
        let mut d_avg = 0.0;
        for i in 1..self.previous_pts.len() {
            let d = derivative(
                self.previous_pts.get(i).unwrap(),
                self.previous_pts.get(i - 1).unwrap(),
            );
            d_avg += d;
        }
        d_avg / (self.previous_pts.len() - 1) as f64
    }
}

fn derivative(a: &Point, b: &Point) -> f64 {
    let z_diff = b.z - a.z;
    let x_d = z_diff / (b.x - a.x);
    let y_d = z_diff / (b.y - a.y);
    if x_d.is_infinite() {
        return x_d;
    }
    if y_d.is_infinite() {
        return x_d;
    }
    let result = (x_d * x_d + y_d * y_d).sqrt();
    if f64::is_infinite(result) {
        println!(
            "d == infi, {}, {}, {}, {} - {}",
            (b.z - a.z),
            (b.x - a.x),
            (b.y - a.y),
            b.x,
            a.x,
        );
    }
    result
}

fn process_band(band: &mut Band, curr_pt: &Point, result: &mut Vec<Point>) {
    band.add_pt(curr_pt);

    if band.last_pt.is_none() {
        band.last_pt = Some(curr_pt.clone());
        return;
    }

    let d = band.previous_pts_derivative();
    if f64::is_infinite(d) {
        return;
    }
    if d.abs() > 85000.0 {
        println!(
            "d: #{}, {}, {}, {}",
            band.index,
            d,
            curr_pt.x - band.last_pt.unwrap().x,
            curr_pt.z - band.last_pt.unwrap().z
        );
        result.push(band.last_pt.unwrap().clone());
        result.push(curr_pt.clone());
    }

    band.boundary_detect(curr_pt);
    if band.in_boundary {
        // result.push(band.last_pt.unwrap().clone());
        // result.push(pt.clone());
    }
    band.last_pt = Some(curr_pt.clone());
}
