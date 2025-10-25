impl Interpolatable for BoneScaleKeyFrame
{
    type Value = Vector2<f32>;

    fn time(&self) -> f32
    {
        self.time
    }

    fn get_value(&self) -> Self::Value
    {
        self.get_scale()
    }
}
