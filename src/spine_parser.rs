use crate::skeleton::Skeleton;

pub trait SpineParser
{
    fn parse(data: &str) -> Result<Skeleton, String>;
}

struct ConcreteSpineParser
{

}

impl SpineParser for ConcreteSpineParser
{
    fn parse(data: &str) -> Result<Skeleton, String>
    {
        Err("Not implemented".to_string())
    }
}
