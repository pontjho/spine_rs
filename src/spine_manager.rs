use crate::skeleton::Skeleton;
use crate::attachment::Attachment;

pub trait SpineManager
{
    fn get_attachments_at(&self, time: f32, from_skeleton: &Skeleton, with_skin: &str) -> Vec<Attachment>;
}

struct ConcreteSpineManager
{

}

impl SpineManager for ConcreteSpineManager
{
    fn get_attachments_at(&self, time: f32, from_skeleton: &Skeleton, with_skin: &str) -> Vec<Attachment>
    {
        vec![]
    }
}
