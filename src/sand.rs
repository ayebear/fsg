use crate::utils::Sandbox;
use bevy::prelude::*;

pub struct SandPlugin;

impl Plugin for SandPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Sandbox::new(800, 600))
            .add_systems(Update, (simulate, render).chain());
    }
}

fn simulate() {}

fn render() {}
