use crate::animation_utilities::spine_vertex::SpineVertex;

#[derive(Debug, Clone)]
pub struct ModelImage
{
    pub transform: [[f32; 4]; 4],
    pub dimensions: (f32, f32),
    pub texture_name: String,
    pub vertices: Vec<SpineVertex>, // This is to allow for freeform deformation
    pub indices: Vec<u32>
}
