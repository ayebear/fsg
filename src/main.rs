pub mod consts;
pub mod element;
pub mod sand;
pub mod sandbox;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use consts::{HEIGHT, WIDTH};
use sand::SandPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "Falling Sand Game".into(),
                    resolution: WindowResolution::new(WIDTH as f32, HEIGHT as f32),
                    present_mode: PresentMode::AutoVsync, // turn on vsync
                    // present_mode: PresentMode::AutoNoVsync, // turn off vsync
                    ..default()
                }),
                ..default()
            }),
            SandPlugin,
        ))
        .run();
}
