use crate::spine_model::SpineModel;
use crate::attachment::Attachment;

pub trait SpineManager
{
    fn get_attachments_at(&self, time: f32, from_model: &SpineModel, with_skin: &str) -> Vec<Attachment>;
}

pub struct ConcreteSpineManager
{

}

impl SpineManager for ConcreteSpineManager
{
    fn get_attachments_at(&self, time: f32, from_model: &SpineModel, with_skin: &str) -> Vec<Attachment>
    {
        vec![]
    }
}
