use crate::{consts::BRUSH_RADIUS, element::Element};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Sandbox {
    pub size: IVec2,
    pub elements: Vec<Element>,
    pub image_handle: Handle<Image>,
}

impl Sandbox {
    pub fn new(width: u32, height: u32, image_handle: Handle<Image>) -> Self {
        Self {
            size: IVec2::new(width as i32, height as i32),
            elements: vec![Element::Empty; width as usize * height as usize],
            image_handle,
        }
    }

    pub fn draw(&mut self, position: IVec2, element: Element) {
        for y in position.y - BRUSH_RADIUS..position.y + BRUSH_RADIUS {
            for x in position.x - BRUSH_RADIUS..position.x + BRUSH_RADIUS {
                if x < 0
                    || x > self.size.x
                    || y < 0
                    || y > self.size.y
                    || position.distance_squared(IVec2::new(x, y)) > BRUSH_RADIUS * BRUSH_RADIUS
                {
                    continue;
                }
                let i = (y * self.size.x + x) as usize;
                self.elements[i] = element;
            }
        }
    }
}
