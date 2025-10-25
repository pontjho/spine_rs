use serde::{Deserialize, Serialize};

use super::BoundingBoxAttachment;
use super::ClippingAttachment;
use super::LinkedMeshAttachment;
use super::MeshAttachment;
use super::PathAttachment;
use super::PointAttachment;
use super::RegionAttachment;

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
