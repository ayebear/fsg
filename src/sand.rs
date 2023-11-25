use crate::{
    consts::{HEIGHT, WIDTH},
    element::Element,
    sandbox::Sandbox,
};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (input, simulate, render).chain());
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    let image = Image::new_fill(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
    );
    let image_handle = images.add(image);
    let mut sandbox = Sandbox::new(WIDTH, HEIGHT, image_handle.clone());
    sandbox.elements[10000] = Element::Wall;
    commands.insert_resource(sandbox);
    commands.spawn(SpriteBundle {
        texture: image_handle,
        ..default()
    });
}

fn input(
    mut sandbox: ResMut<Sandbox>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // Get mapped mouse position
    let window = q_windows.single();
    let (camera, camera_transform) = q_cameras.single();
    let Some(mouse_position) = window.cursor_position() else {
        return;
    };
    let Some(position) = camera.viewport_to_world_2d(camera_transform, mouse_position) else {
        return;
    };
    let position = Vec2::new(
        position.x + WIDTH as f32 / 2.,
        HEIGHT as f32 - (position.y + HEIGHT as f32 / 2.),
    )
    .round()
    .as_ivec2();
    // Use mouse position to draw/erase
    if mouse_input.pressed(MouseButton::Left) {
        // todo: store a tool with element and use keyboard 1-9 to switch tools
        sandbox.paint(position, Element::Sand);
    } else if mouse_input.pressed(MouseButton::Right) {
        sandbox.paint(position, Element::Empty);
    }
}

fn simulate(mut sandbox: ResMut<Sandbox>) {
    sandbox.simulate();
}

fn render(sandbox: Res<Sandbox>, mut images: ResMut<Assets<Image>>) {
    let image = images.get_mut(sandbox.image_handle.clone()).unwrap();
    sandbox.render(image);
}
