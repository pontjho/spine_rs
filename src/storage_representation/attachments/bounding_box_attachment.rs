use serde::Deserialize;
use crate::util::ffffffff;
use crate::util::deserialize_colour;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoundingBoxAttachment
{
    #[serde(alias = "vertexCount")]
    pub vertex_count: usize,
    pub vertices: Vec<f32>,

    #[serde(default="ffffffff", deserialize_with="deserialize_colour")]
    pub color: u32
}
