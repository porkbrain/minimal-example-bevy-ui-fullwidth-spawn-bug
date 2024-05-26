mod dialog;

use bevy::{input::common_conditions::input_just_pressed, prelude::*, render::view::RenderLayers};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{PixelViewport, PixelZoom};

/// `#0d0e1f`
pub const PRIMARY_COLOR: Color = Color::rgb(0.050980393, 0.05490196, 0.12156863);

fn main() {
    App::new()
        .insert_resource(ClearColor(PRIMARY_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some({
                        let mut w = Window {
                            title: "bevy_ui_full_width_spawn".into(),
                            ..default()
                        };

                        w.set_maximized(true);
                        w
                    }),
                    ..default()
                }),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            dialog::toggle_spawn.run_if(input_just_pressed(KeyCode::Space)),
        )
        .run()
}

/// System to spawn 2D camera with component [`MainCamera`].
pub fn spawn_camera(mut cmd: Commands) {
    cmd.spawn((
        Name::from("Main camera"),
        PixelZoom::Fixed(3),
        PixelViewport,
        RenderLayers::from_layers(&[0, 1, 2]),
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                order: 1,
                ..default()
            },
            ..default()
        },
    ));
}
