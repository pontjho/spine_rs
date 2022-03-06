pub struct IKConstraint
{

    pub name: String,
    pub order: usize,
    pub skin: bool,
    pub bones: Vec<String>,
    pub target: String,
    pub mix: f32,
    pub softness: f32,
    pub bend_positive: bool,
    pub compress: bool,
    pub stretch: bool,
    pub uniform: bool
}
