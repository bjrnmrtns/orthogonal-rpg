use crate::transform::Transform;

pub struct Components {
    pub camera: usize,
    pub player: usize,
    pub input_state: Vec<Option<bool>>,
    pub transform_component: Vec<Option<Transform>>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            camera: 0,
            player: 1,
            input_state: Vec::new(),
            transform_component: Vec::new(),
        }
    }
}
