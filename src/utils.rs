use crate::element::Element;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Sandbox {
    pub size: UVec2,
    pub elements: Vec<Element>,
}

impl Sandbox {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size: UVec2::new(width as u32, height as u32),
            elements: vec![Element::Empty; width * height],
        }
    }
}
