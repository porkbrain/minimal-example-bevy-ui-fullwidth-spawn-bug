use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Component)]
pub struct DialogCamera;
#[derive(Component)]
pub struct DialogUiRoot;
#[derive(Component)]
pub struct DialogText;
#[derive(Component)]
pub struct DialogPortrait;

/// If the UI is already already spawned then despawn it.
///
/// Otherwise spawn it.
pub fn toggle_spawn(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,

    camera: Query<Entity, With<DialogCamera>>,
    root: Query<Entity, With<DialogUiRoot>>,
) {
    if let Ok(camera) = camera.get_single() {
        cmd.entity(camera).despawn_recursive();
        cmd.entity(root.single()).despawn_recursive();
        return;
    }

    let camera = cmd
        .spawn((
            Name::from("Portrait dialog camera"),
            DialogCamera,
            RenderLayers::layer(25),
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    order: 11,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let text = Text::from_section(
        "Test",
        TextStyle {
            color: Color::WHITE,
            ..default()
        },
    );

    let root = cmd
        .spawn((
            Name::new("Portrait dialog root"),
            DialogUiRoot,
            TargetCamera(camera),
            RenderLayers::layer(25),
            NodeBundle {
                // centers the content
                style: Style {
                    width: Val::Vw(100.0),
                    bottom: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::RowReverse,

                    ..default()
                },
                ..default()
            },
        ))
        .id();

    cmd.entity(root).with_children(|parent| {
        parent
            .spawn((
                Name::new("Dialog Box"),
                RenderLayers::layer(25),
                ImageBundle {
                    image: UiImage::new(asset_server.load("dialog_box.png")),
                    style: Style {
                        width: Val::Px(350.0 * 3 as f32),
                        height: Val::Px(107.0 * 3 as f32),
                        margin: UiRect {
                            left: Val::Px(0.0),
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    // a `NodeBundle` is transparent by default, so to see
                    // the image we have to set its color to `WHITE`
                    background_color: Color::WHITE.into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                // of the whole box, 70% is usable space for
                // text and choices
                const TEXT_HEIGHT_PERCENT: f32 = 23.0;
                const TEXT_MARGIN_TOP_PERCENT: f32 = 2.0;

                parent.spawn((
                    DialogText,
                    Name::new("Dialog text"),
                    RenderLayers::layer(25),
                    TextBundle {
                        text,
                        style: Style {
                            width: Val::Percent(90.0),
                            height: Val::Percent(TEXT_HEIGHT_PERCENT),
                            margin: UiRect {
                                top: Val::Percent(TEXT_MARGIN_TOP_PERCENT),
                                bottom: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    },
                ));
            });

        parent.spawn((
            DialogPortrait,
            Name::new("Portrait"),
            RenderLayers::layer(25),
            ImageBundle {
                style: Style {
                    width: Val::Px(384.0),
                    height: Val::Px(384.0),
                    margin: UiRect {
                        right: Val::Px(0.0),
                        left: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    ..default()
                },
                image: UiImage::new(asset_server.load("marie1.png")),
                ..default()
            },
        ));
    });
}
