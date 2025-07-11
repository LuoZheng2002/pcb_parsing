
#[derive(Debug, Clone)]
pub enum PadShape {
    Circle {
        diameter: f32,
    },
    Square {
        side_length: f32,
    },
    Rectangle {
        width: f32,
        height: f32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PadID(pub String);

#[derive(Debug, Clone)]
pub struct Pad {
    pub id: PadID,
    pub position: (f64, f64),
    pub shape: PadShape,
    pub rotation: cgmath::Deg<f32>, // Rotation in degrees
    pub clearance: f32,             // Clearance around the pad
}
