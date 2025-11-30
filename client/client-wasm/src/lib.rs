mod utils;

use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitSettings;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn start_render() {
    bevy_app();
}

fn bevy_app() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#game".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, jaja)
        .run();
}

fn jaja(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 0., 0.),
            custom_size: Some(Vec2::new(200., 100.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
