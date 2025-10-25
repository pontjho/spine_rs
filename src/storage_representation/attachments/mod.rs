mod attachment;
mod attachment_type;
mod region_attachment;
mod mesh_attachment;
mod linked_mesh_attachment;
mod clipping_attachment;
mod point_attachment;
mod path_attachment;
mod bounding_box_attachment;

pub use attachment::Attachment;
pub use attachment_type::AttachmentType;
pub use region_attachment::RegionAttachment;
pub use mesh_attachment::MeshAttachment;
pub use linked_mesh_attachment::LinkedMeshAttachment;
pub use clipping_attachment::ClippingAttachment;
pub use point_attachment::PointAttachment;
pub use path_attachment::PathAttachment;
pub use bounding_box_attachment::BoundingBoxAttachment;
