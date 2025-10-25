use serde::Deserialize;
use serde::Serialize;

use crate::util::ffffffff;
use crate::util::deserialize_colour;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct MeshAttachment
{
    pub path: String,
    pub uvs: String,
    pub triangles: String,
    pub vertices: String,
    pub hull: String,
    pub edges: String,

    #[serde(default="ffffffff", deserialize_with="deserialize_colour")]
    pub color: u32
}
