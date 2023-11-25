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

    pub fn draw(&mut self, position: IVec2, element: Element) {
        let position = position.clamp(IVec2::ZERO, self.size.as_ivec2()).as_uvec2();
        let i = (position.y * self.size.x + position.x) as usize;
        self.elements[i] = element;
    }
}
