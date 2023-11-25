// use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Element {
    Empty,
    Wall,
    Sand,
    // Water,
    // Plant,
    // Acid,
    // Fire,
    // Oil,
    // Steam,
    // Lava,
}

impl Element {
    pub const fn get_color(&self) -> (u8, u8, u8) {
        match self {
            Element::Empty => (0, 0, 0),
            Element::Wall => (128, 128, 128),
            Element::Sand => (192, 128, 32),
        }
    }
}
