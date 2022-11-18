use crate::transform::Transform;

pub enum WindowEvent {
    Resized(u32, u32),
}

pub enum InputEvent {
    KeyPressed,
}

pub struct World {
    pub camera: usize,
    pub player: usize,
    pub input_state: Vec<Option<bool>>,
    pub transform_component: Vec<Option<Transform>>,
    pub window_events: Vec<WindowEvent>,
    pub input_events: Vec<WindowEvent>,
}

impl World {
    pub fn new() -> Self {
        Self {
            camera: 0,
            player: 1,
            input_state: Vec::new(),
            transform_component: Vec::new(),
            window_events: Vec::new(),
            input_events: Vec::new(),
        }
    }
}
