use std::collections::HashMap;

use cgmath::Matrix4;

use crate::{animation_utilities::{spine_vertex::SpineVertex, ModelImage}, bounding_box::BoundingBox, storage_representation::{animations::Animation, SpineModel}};

pub trait SpineManager: Sync + Send
{
    fn get_bone_transforms(&self, time: f32, model: &SpineModel, animation: &Animation) -> HashMap<String, Matrix4<f32>>;
    fn get_attachments_at(&self, time: f32, from_model: &SpineModel, with_animation: &str, with_skin: &str) -> Vec<ModelImage>;
    fn get_animation_id_attachments_at(&self, time: f32, from_model: &SpineModel, with_animation: usize, with_skin: &str) -> Vec<ModelImage>;

    fn mix_animations(&self, animations: &Vec<&Animation>) -> Animation;
    fn get_attachments_for_animation(&self, time: f32, from_model: &SpineModel, with_animation: &Animation, with_skin: &str) -> Vec<ModelImage>;
    fn get_bounding_boxes_for_animation(&self, time: f32, from_model: &SpineModel, with_animation: &Animation, with_skin: &str) -> Vec<BoundingBox>;
}

pub fn dimensions_as_vertices(dimensions: (f32, f32), padding: [f32; 4]) -> [([f32; 3], [f32; 2]); 6]
{
    let (w, h) = dimensions;
    let (half_width, half_height) = (w / 2.0, h / 2.0);

    let [left_padding, top_padding, right_padding, bottom_padding] = padding;

    let denominator = 1.0;//2500.0;
    let (left, right) = (-half_width + left_padding, half_width - right_padding);
    let (top, bottom) = (half_height - top_padding, -half_height + bottom_padding);

    let left = left / denominator;
    let right = right / denominator;
    let top = top / denominator;
    let bottom = bottom / denominator;

    let u_left = 0.0;
    let u_right = 1.0;
    let u_top = 0.0;
    let u_bottom = 1.0;
    
    let vertices = [

        ([left, top, 0.0], [u_left, u_top]),
        ([left, bottom, 0.0], [u_left, u_bottom]),
        ([right, bottom, 0.0], [u_right, u_bottom]),
        ([right, bottom, 0.0], [u_right, u_bottom]),
        ([right, top, 0.0], [u_right, u_top]),
        ([left, top, 0.0], [u_left, u_top]),
    ];

    // let v1 = [
    //     [-0.5, 0.5, 0.0],
    //     [-0.5, -0.5, 0.0],
    //     [0.5, -0.5, 0.0],
    //     [0.5, -0.5, 0.0],
    //     [0.5, 0.5, 0.0],
    //     [-0.5, 0.5, 0.0],];
    vertices
}

// pub fn dimensions_as_vertices_bottom_left_aligned(dimensions: (f32, f32), _padding: [f32; 4]) -> [[f32; 3]; 4]
// {
//     let (w, h) = dimensions;
//     let (half_width, _half_height) = (w / 2.0, h / 2.0);
//     let (left, right) = (-half_width, half_width);
//     let (top, bottom) = (h, 0.0);
//     let vertices = [
//         [left, top, 1.0],
//         [right, top, 1.0],
//         [right, bottom, 1.0],
//         [left, bottom, 1.0]
//     ];
//     vertices
// }
