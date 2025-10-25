use serde::{Deserialize, Serialize};
use crate::util::troo;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct PathAttachment
{
    #[serde(default)]
    pub closed: bool,
    #[serde(default="troo")]
    pub constant_speed: bool,
    pub lengths: bool,
    pub vertex_count: bool,
    pub vertices: bool
}
