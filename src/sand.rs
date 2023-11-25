use crate::{
    consts::{HEIGHT, WIDTH},
    element::Element,
    utils::Sandbox,
};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (simulate, render).chain());
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

fn simulate(mut sandbox: ResMut<Sandbox>) {}

fn render(sandbox: Res<Sandbox>, mut images: ResMut<Assets<Image>>) {
    let image = images.get_mut(sandbox.image_handle.clone()).unwrap();
    sandbox
        .elements
        .iter()
        .enumerate()
        .for_each(|(i, element)| {
            let color = element.get_color();
            let img_i = i * 4;
            image.data[img_i] = color.0;
            image.data[img_i + 1] = color.1;
            image.data[img_i + 2] = color.2;
            image.data[img_i + 3] = 255;
        });
}
