use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[typetag::serde(tag = "type")]
trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
struct Circle {
    r: f64,
}
#[typetag::serde]
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

#[test]
fn test_cbor() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { a: 3.0, b: 4.0 }),
        Box::new(Circle { r: 2.0 }),
    ];

    assert_eq!(shapes[0].area(), 3.0 * 4.0);
    assert_eq!(shapes[1].area(), 2.0 * 2.0 * std::f64::consts::PI);

    let mut buffer = Vec::new();
    ciborium::ser::into_writer(&shapes, &mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);
    let deserialized_shapes: Vec<Box<dyn Shape>> = ciborium::de::from_reader(&mut cursor).unwrap();

    assert_eq!(deserialized_shapes[0].area(), 3.0 * 4.0);
    assert_eq!(
        deserialized_shapes[1].area(),
        2.0 * 2.0 * std::f64::consts::PI
    );
}
