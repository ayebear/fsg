use crate::{consts::BRUSH_RADIUS, element::Element};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Sandbox {
    pub size: IVec2,
    pub elements: Vec<Element>,
    pub image_handle: Handle<Image>,
    pub tool: Element,
}

impl Sandbox {
    pub fn new(width: u32, height: u32, image_handle: Handle<Image>) -> Self {
        Self {
            size: IVec2::new(width as i32, height as i32),
            elements: vec![Element::Empty; width as usize * height as usize],
            image_handle,
            tool: Element::Sand,
        }
    }

    pub fn paint(&mut self, position: IVec2, element: Element) {
        for y in position.y - BRUSH_RADIUS..position.y + BRUSH_RADIUS {
            for x in position.x - BRUSH_RADIUS..position.x + BRUSH_RADIUS {
                if position.distance_squared(IVec2::new(x, y)) > BRUSH_RADIUS * BRUSH_RADIUS {
                    continue;
                }
                let i = y.rem_euclid(self.size.y) * self.size.x + x.rem_euclid(self.size.x);
                self.elements[i as usize] = element;
            }
        }
    }

    pub fn simulate(&mut self) {
        for y in (0..self.size.y).rev() {
            for x in 0..self.size.x {
                let i = y * self.size.x + x;
                let element = self.elements[i as usize];
                match element {
                    Element::Sand => {
                        let ii = (y + 1).rem_euclid(self.size.y) * self.size.x + x;
                        let element_below = self.elements[ii as usize];
                        match element_below {
                            Element::Empty => {
                                // move down
                                self.elements.swap(i as usize, ii as usize);
                            }
                            _ => {
                                // collide
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn render(&self, image: &mut Image) {
        self.elements.iter().enumerate().for_each(|(i, element)| {
            let color = element.get_color();
            let img_i = i * 4;
            image.data[img_i] = color.0;
            image.data[img_i + 1] = color.1;
            image.data[img_i + 2] = color.2;
            image.data[img_i + 3] = 255;
        });
    }
}
