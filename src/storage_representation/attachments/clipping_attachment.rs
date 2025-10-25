use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ClippingAttachment
{
    pub end: String,
    pub vertex_count: usize,
    pub vertices: Vec<f32>
}