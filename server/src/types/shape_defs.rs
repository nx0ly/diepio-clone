use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum ShapeType {
    Egg,
    Square,
    Triangle,
    Pentagon,
    Hexagon,
    AlphaPentagon,
    AlphaHexagon,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShapeDef {
    #[serde(rename = "type")]
    pub shape_type: ShapeType,
    pub vertices: Vec<(f32, f32)>,
}
