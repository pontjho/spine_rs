pub struct Attachment
{
    pub name: String,
    pub attachment_type: AttachmentType
}

pub enum AttachmentType
{
    Region(RegionAttachment),
    Mesh(MeshAttachment),
    LinkedMesh(LinkedMeshAttachment),
    BoundingBox(BoundingBoxAttachment),
    Path(PathAttachment),
    Point(PointAttachment),
    Clipping(ClippingAttachment)
}

pub struct RegionAttachment
{
    pub path: String,
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
    pub color: f32
}

pub struct MeshAttachment
{
    pub path: String,
    pub uvs: String,
    pub triangles: String,
    pub vertices: String,
    pub hull: String,
    pub edges: String,
    pub color: String,
    pub width: String,
    pub height: String
}

pub struct LinkedMeshAttachment
{
    pub path: String,
    pub skin: String,
    pub parent: String,
    pub deform: String,
    pub color: String,
    pub width: String,
    pub height: String
}

pub struct BoundingBoxAttachment
{
    pub vertex_count: String,
    pub vertices: String,
    pub color: String
}

pub struct PathAttachment
{
    pub closed: bool,
    pub constant_speed: bool,
    pub lengths: bool,
    pub vertex_count: bool,
    pub vertices: bool,
    pub color: bool
}

pub struct PointAttachment
{
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub color: f32
}

pub struct ClippingAttachment
{
    pub end: String,
    pub vertex_count: String,
    pub vertices: String
}
