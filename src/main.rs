pub mod element;
pub mod sand;
pub mod utils;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use sand::SandPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "Falling Sand Game".into(),
                    resolution: WindowResolution::new(800., 600.),
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
