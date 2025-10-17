use bevy::core_pipeline::core_2d::Camera2d;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_svg::prelude::*;
// use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hsv(1.0, 0.0, 0.80)));
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GeoQuizz Belgium".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }));
        app.add_plugins(SvgPlugin);
        app.add_systems(Startup, setup);
        // app.add_systems(Update, camera_zoom_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("belgium_map.svg");
    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn((
        Svg2d(svg),
        Origin::Center, // Origin::TopLeft is the default
    ));
}

pub fn camera_zoom_system(
    mut evr_scroll: EventReader<MouseWheel>,
    mut camera: Query<(Option<Mut<Projection>>, Mut<Transform>), With<Camera>>,
) {
    for ev in evr_scroll.read() {
        for (projection, mut transform) in camera.iter_mut() {
            let amount = match ev.unit {
                MouseScrollUnit::Line => ev.y,
                MouseScrollUnit::Pixel => ev.y,
            };
            if let Some(mut projection) = projection {
                if let Projection::Orthographic(ref mut projection) = *projection {
                    projection.scale -= if projection.scale <= 1.0 {
                        amount * 0.05
                    } else {
                        amount
                    };
                    projection.scale = projection.scale.clamp(0.01, 10.0);
                }
            } else {
                transform.translation.z -= amount;
            }
        }
    }
}
