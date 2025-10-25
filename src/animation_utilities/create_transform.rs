use cgmath::{Deg, Matrix4, Vector2, Vector3};



pub fn create_transform(rotation: Deg<f32>, translation: Vector3<f32>, scale: Vector2<f32>) -> Matrix4<f32>
{
    let the_return = 
            Matrix4::from_scale(1.0)
            * Matrix4::from_angle_z(rotation)
            * Matrix4::from_nonuniform_scale(scale[0], scale[1], 1.0)
            * Matrix4::from_translation(translation)
            * Matrix4::from_scale(1.0);
    //println!("Creating bone {} transform from translation {:?} and rotation {:?} as {:?}", bone.name, translation, rotation, the_return);
    the_return
}
