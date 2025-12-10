use std::{cmp::Ordering, collections::HashMap};

use cgmath::Matrix4;

use crate::{animation_utilities::{model_image::ModelImage, spine_animation_helper::SpineAnimationHelper, spine_manager::{dimensions_as_vertices, SpineManager}, spine_vertex::SpineVertex}, storage_representation::{animations::Animation, attachments::AttachmentType, bones::BoneKeyFrame, slots::SlotKeyFrame, SpineModel}};

pub struct ConcreteSpineManager
{
    pub animator: Box<dyn SpineAnimationHelper>
}

impl ConcreteSpineManager
{

    fn get_active_attachments(&self, time: f32, model: &SpineModel, animation: &Animation, with_skin: &str) -> Vec<(String, String, AttachmentType)>
    {
        let skin = model.skins.iter().find(|s| s.name == with_skin).unwrap();
        let skin_attachments = &skin.attachments;
        let active_attachments: Vec<_> = model
            .slots
            .iter()
            .map(|v| (v.name.clone(), v.bone.clone(), self.animator.get_slot_attachment(v, animation, time)))
            .filter_map(|(slot_name, bone_name, attachment_name)| attachment_name.map(|aname| (slot_name, bone_name, aname)))
            .map(|(slot_name, bone_name, attachment_name)| (bone_name, attachment_name.clone(), skin_attachments[&slot_name][&attachment_name].clone()))
            .collect();
        active_attachments
    }

    fn get_attachment_transforms(&self, active_attachments: Vec<(String, String, AttachmentType)>, bone_global_transforms: HashMap<String, Matrix4<f32>>) -> Vec<(String, Matrix4<f32>, (f32, f32))>
    {
        let attachment_transforms = active_attachments
            .into_iter()
            .filter_map(|(bone_name, attachment_name, attachment)| match attachment {
                AttachmentType::Region(a) => {
                    let bone_transform: Matrix4<f32> = bone_global_transforms[&bone_name];
                    let attachment_transform: Matrix4<f32> = a.get_transform(&bone_transform, &attachment_name);
                    let image_dimensions = (a.width, a.height);
                    Some((a.path.unwrap_or(attachment_name), attachment_transform, image_dimensions))
                },
                _ => None
            })
            .collect::<Vec<_>>();

        attachment_transforms
    }

    fn get_attachment_images(&self, attachment_transforms: Vec<(String, Matrix4<f32>, (f32, f32))>) -> Vec<ModelImage>
    {
        const INDICES: [u32; 6] = [0, 1, 4, 1, 3, 4];
        let images = attachment_transforms
            .into_iter()
            .map(|(texture_name, attachment_transform, image_dimensions)|{
                let vertices = dimensions_as_vertices(image_dimensions, [0., 0., 0., 0.])
                    .into_iter()
                    //.enumerate()
                    .map(|(position, uv)| SpineVertex { position, uv });
                ModelImage {
                    transform: attachment_transform.into(),
                    texture_name,
                    dimensions: image_dimensions,
                    vertices: vertices.into_iter().collect(),
                    indices: INDICES.clone().into_iter().collect()
                }
            })
            .collect();
        images
    }
}


impl SpineManager for ConcreteSpineManager
{
    fn get_bone_transforms(&self, time: f32, model: &SpineModel, animation: &Animation) -> HashMap<String, Matrix4<f32>>
    {
        let anim_length = animation
            .bones
            .iter()
            .map(|(_, anim)| anim
                .rotate
                .iter()
                .map(|r| r.time)
                .chain(anim.translate.iter().map(|t| t.time))
                .chain(anim.scale.iter().map(|t| t.time))
                .chain(anim.shear.iter().map(|t| t.time))
            )
            .flatten()
            // .collect();
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0);
        let time = time * anim_length;
        
        let temp_debug = model
            .bones
            .iter()
            .map(|bone| (bone.name.clone(), bone.parent.clone(), self.animator.get_bone_transform(bone, animation, time)))
            .collect::<Vec<_>>();
        let the_return: HashMap<String, Matrix4<f32>> = temp_debug.iter()
            .fold(HashMap::default(), |accum, (bone_name, parent_bone, transform)| {
                let bone_transform = parent_bone.as_ref().map(|b| accum[b] * transform).unwrap_or(transform.clone());
                let new_transform = vec![(bone_name.to_string(), bone_transform)];
                accum.into_iter().chain(new_transform).collect()
            });
        the_return
    }
    
    fn get_animation_id_attachments_at(&self, time: f32, model: &SpineModel, animation_id: usize, with_skin: &str) -> Vec<ModelImage>
    {
        let animation_name = model.animations.iter().nth(animation_id).unwrap().0;
        let the_return = self.get_attachments_at(time, model, animation_name, with_skin);
        the_return
    }

    fn get_attachments_at(&self, time: f32, model: &SpineModel, animation_name: &str, with_skin: &str) -> Vec<ModelImage>
    {
        let animation = &model.animations[animation_name];
        self.get_attachments_for_animation(time, model, animation, with_skin)
    }

    fn mix_animations(&self, animations: &Vec<&Animation>) -> Animation
    {
        let mut bones: HashMap<String, BoneKeyFrame> = Default::default();
        let mut slots: HashMap<String, SlotKeyFrame> = Default::default();
        let events = animations.iter().flat_map(|a| a.events.clone()).collect();

        for Animation { bones: a_bones, slots: a_slots , events } in animations
        {
            for (bone_name, bone) in a_bones
            {
                if !bones.contains_key(bone_name)
                {
                    bones.insert(bone_name.to_string(), bone.clone());
                }
            }
            for (slot_name, slot) in a_slots
            {
                if !slots.contains_key(slot_name)
                {
                    slots.insert(slot_name.to_string(), slot.clone());
                }
            }
        }

        Animation {
            bones,
            slots,
            events
        }
    }

    fn get_attachments_for_animation(&self, time: f32, model: &SpineModel, animation: &Animation, with_skin: &str) -> Vec<ModelImage>
    {
        let bone_global_transforms = self.get_bone_transforms(time, model, animation);
        let active_attachments: Vec<_> = self.get_active_attachments(time, model, animation, with_skin);
        let attachment_transforms = self.get_attachment_transforms(active_attachments, bone_global_transforms);
        let images: Vec<_> = self.get_attachment_images(attachment_transforms);
        images
    }
    
    fn get_bounding_boxes_for_animation(&self, time: f32, from_model: &SpineModel, with_animation: &Animation, with_skin: &str) -> Vec<crate::bounding_box::BoundingBox> {
        todo!()
    }
}
