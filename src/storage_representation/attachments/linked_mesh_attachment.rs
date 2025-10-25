use serde::{Deserialize, Serialize};
use crate::util::ffffffff;
use crate::util::deserialize_colour;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct LinkedMeshAttachment
{
    pub path: String,
    pub skin: String,
    pub parent: String,
    pub deform: String,

    #[serde(default="ffffffff", deserialize_with="deserialize_colour")]
    pub color: u32
}
