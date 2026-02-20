#[derive(Debug)]
pub enum BoundaryType {
    WrapAround,
    SoftBoundry,
}

#[derive(Debug)]
pub struct FlockParams {
    pub perception_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub boundary_type: BoundaryType,
}

impl FlockParams {
    pub fn new() -> FlockParams {
        FlockParams {
            perception_radius: 10.0,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            max_speed: 1.0,
            max_force: 1.0,
            boundary_type: BoundaryType::WrapAround,
        }
    }
}
