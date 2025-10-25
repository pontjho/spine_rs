pub trait Interpolatable
{
    type Value: std::ops::Mul<f32, Output = Self::Value> + std::ops::Add<Self::Value, Output = Self::Value> + core::fmt::Debug;

    fn time(&self) -> f32;
    fn get_value(&self) -> Self::Value;
}
