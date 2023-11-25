use crate::element::Element;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Sandbox {
    pub size: UVec2,
    pub elements: Vec<Element>,
    pub image_handle: Handle<Image>,
}

impl Sandbox {
    pub fn new(width: u32, height: u32, image_handle: Handle<Image>) -> Self {
        Self {
            size: UVec2::new(width, height),
            elements: vec![Element::Empty; width as usize * height as usize],
            image_handle,
        }
    }
}
