use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[typetag::serde(tag = "type")]
trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug, Deserialize, Decode, Serialize, Encode)]
struct Rectangle {
    a: f64,
    b: f64,
}
#[typetag::serde]
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
}

#[derive(Debug, Deserialize, Decode, Serialize, Encode)]
struct Circle {
    r: f64,
}
#[typetag::serde]
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { a: 3.0, b: 4.0 }),
        Box::new(Circle { r: 2.0 }),
    ];

    // Print original areas
    println!("Original shapes:");
    for shape in shapes.iter() {
        println!("Area: {}", shape.area());
    }

    // Serialize shapes
    let mut buffer = Vec::new();
    ciborium::ser::into_writer(&shapes, &mut buffer).unwrap();
    println!("\nSerialized shapes: {:?}", buffer);

    // Deserialize shapes
    let mut cursor = Cursor::new(buffer);
    let deserialized_shapes: Vec<Box<dyn Shape>> = ciborium::de::from_reader(&mut cursor).unwrap();

    // Print deserialized areas
    println!("\nDeserialized shapes:");
    for shape in deserialized_shapes.iter() {
        println!("Area: {}", shape.area());
    }
}
