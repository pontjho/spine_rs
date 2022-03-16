use serde::{Serialize, Deserialize};
use crate::util::one;
use crate::util::ffffffff;
use crate::util::troo;
use crate::util::deserialize_colour;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Attachment
{
    pub name: String,
    #[serde(alias = "type")]
    pub attachment_type: AttachmentType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
//#[serde(tag="type")]
#[serde(untagged)]
pub enum AttachmentType
{
    Region(RegionAttachment),
    Mesh(MeshAttachment),
    LinkedMesh(LinkedMeshAttachment),
    #[serde(alias = "boundingbox")]
    BoundingBox(BoundingBoxAttachment),
    Path(PathAttachment),
    Point(PointAttachment),
    Clipping(ClippingAttachment)
}

// impl Default for AttachmentType
// {
//     fn default() -> Self
//     {
//         AttachmentType::Region
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct RegionAttachment
{
    pub path: Option<String>,

    #[serde(default)]
    pub x: f32,

    #[serde(default)]
    pub y: f32,

    #[serde(default="one")]
    pub scale_x: f32,

    #[serde(default="one")]
    pub scale_y: f32,

    #[serde(default)]
    pub rotation: f32,
    pub width: f32,
    pub height: f32,

    #[serde(default="ffffffff", deserialize_with="deserialize_colour")]
    pub color: u32
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoundingBoxAttachment
{
    #[serde(alias = "vertexCount")]
    pub vertex_count: usize,
    pub vertices: Vec<f32>,

    #[serde(default="ffffffff", deserialize_with="deserialize_colour")]
    pub color: u32
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct PointAttachment
{
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct ClippingAttachment
{
    pub end: String,
    pub vertex_count: usize,
    pub vertices: Vec<f32>
}
