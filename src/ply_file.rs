extern crate ply_rs;

use self::ply_rs::ply::{ElementDef, Property, PropertyDef, PropertyType, ScalarType};
use crate::geometry::Point;
use ply_rs::ply::DefaultElement;
use ply_rs::ply::Ply;
use std::any::Any;

pub fn read_ply_file(path: String) -> Ply<DefaultElement> {
    let mut f = std::fs::File::open(path).unwrap();

    // create a parser
    let p = ply_rs::parser::Parser::<ply_rs::ply::DefaultElement>::new();

    // use the parser: read the entire file
    let ply = p.read_ply(&mut f);
    match ply {
        Ok(val) => val,
        Err(_) => {
            panic!()
        }
    }
}

pub fn ply_to_pts(ply: Ply<DefaultElement>) -> Result<Vec<Point>, String> {
    //debug_print(&ply);

    let vertices = match ply.payload.get("vertex") {
        None => {
            panic!()
        }
        Some(val) => val,
    };
    let mut result = vec![];
    for vertex in vertices {
        let point = match vertex_to_point(vertex) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        result.push(point);
    }
    return Ok(result);
}

fn debug_print(ply: &Ply<DefaultElement>) {
    println!("Ply header: {:#?}", ply.header);
    let v1 = ply.payload.get("vertex").unwrap().get(0).unwrap().type_id();
    println!("{:?}", v1);
    let v2 = ply
        .payload
        .get("vertex")
        .unwrap()
        .get(0)
        .unwrap()
        .get("x")
        .unwrap();
    println!("{:?}", v2);
    //println!("Ply data: {:?}", ply.payload);
}

fn vertex_to_point(vertex: &DefaultElement) -> Result<Point, String> {
    let x_property = match vertex.get("x") {
        None => {
            return Err(String::from("vertex missing x"));
        }
        Some(val) => val,
    };
    let x = match x_property {
        Property::Double(val) => val,
        _ => {
            return Err(String::from("vertex x coordinate is not double"));
        }
    };
    let y_property = match vertex.get("y") {
        None => {
            return Err(String::from("vertex missing y"));
        }
        Some(val) => val,
    };
    let y = match y_property {
        Property::Double(val) => val,
        _ => {
            return Err(String::from("vertex y coordinate is not double"));
        }
    };
    let z_property = match vertex.get("z") {
        None => {
            return Err(String::from("vertex missing z"));
        }
        Some(val) => val,
    };
    let z = match z_property {
        Property::Double(val) => val,
        _ => {
            return Err(String::from("vertex z coordinate is not double"));
        }
    };
    Ok(Point {
        x: x.clone(),
        y: y.clone(),
        z: z.clone(),
    })
}

pub fn write_to_ply_file(path: String, data: &Vec<Point>) {
    let mut f = std::fs::File::create(path).unwrap();

    let mut ply = pts_to_ply(data);

    // set up a writer
    let w = ply_rs::writer::Writer::new();
    let written = match w.write_ply(&mut f, &mut ply) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            panic!();
        }
    };
    println!("{} bytes written", written);
}

fn pts_to_ply(data: &Vec<Point>) -> Ply<DefaultElement> {
    let mut ply = Ply::<DefaultElement>::new();
    let x_str = String::from("x");
    let y_str = String::from("y");
    let z_str = String::from("z");
    let vertex_header = String::from("vertex");

    ply.header
        .elements
        .insert(vertex_header.clone(), ply_header(data.len()));

    let mut vertices: Vec<DefaultElement> = vec![];

    for pt in data {
        let mut elem = DefaultElement::new();
        elem.insert(x_str.clone(), Property::Double(pt.x));
        elem.insert(y_str.clone(), Property::Double(pt.y));
        elem.insert(z_str.clone(), Property::Double(pt.z));
        vertices.push(elem);
    }
    ply.payload.insert(vertex_header, vertices);

    ply
}

fn ply_header(data_len: usize) -> ElementDef {
    let x_str = String::from("x");
    let y_str = String::from("y");
    let z_str = String::from("z");
    let vertex_header = String::from("vertex");

    let mut elem_def = ElementDef {
        name: vertex_header,
        count: data_len,
        properties: Default::default(),
    };
    let p_def_x = PropertyDef {
        name: x_str.clone(),
        data_type: PropertyType::Scalar(ScalarType::Double),
    };
    let p_def_y = PropertyDef {
        name: y_str.clone(),
        data_type: PropertyType::Scalar(ScalarType::Double),
    };
    let p_def_z = PropertyDef {
        name: z_str.clone(),
        data_type: PropertyType::Scalar(ScalarType::Double),
    };
    elem_def.properties.insert(x_str, p_def_x);
    elem_def.properties.insert(y_str, p_def_y);
    elem_def.properties.insert(z_str, p_def_z);
    elem_def
}
