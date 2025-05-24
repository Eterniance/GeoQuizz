use bevy::prelude::*;
use bevy::input::{gestures::*,
        mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel}};
use bevy::core_pipeline::core_2d::Camera2d;
// use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

use bevy_svg::prelude::*;

// mod game;

fn main() {
    App::new()
        .add_plugins(MapPlugin)
        .add_plugins(HelloPlugin)
        .run();
}

#[derive(Component, Debug)]
struct Name(String);

#[derive(Component, Debug)]
struct Location(Vec2);

#[derive(Component, Debug)]
struct City;

#[derive(Bundle, Debug)]
struct BundleCity {
    city: City,
    name: Name,
    loc: Location,
}

#[derive(Component, Debug)]
enum Guess {
    Name(String),
    Location(Vec2),
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct LocTimer(Timer);

fn add_city(mut commands: Commands) {
    commands.spawn(BundleCity{
        city: City,
        name: Name("Soignies".to_string()),
        loc: Location(Vec2::new(0.0, 0.0)),
    });
}

fn greet_city(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<City>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {:?}!", name.0);
        }
    }
}

fn find_city(time: Res<Time>, mut timer: ResMut<LocTimer>, query: Query<&Location, With<City>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for loc in &query {
            println!("At {:?}", loc.0);
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("belgium_map.svg");
    commands.spawn((Camera2d::default(), Msaa::Sample4));
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




fn click_in_window(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
) {
    let window = windows.get_single().unwrap(); // assumes one window

    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            if let Some(position) = window.cursor_position() {
                println!("Clicked at position: {:?}", position);

            } else {
                println!("Clicked, but cursor was outside of the window.");
            }
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Once)));
        app.insert_resource(LocTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        app.add_systems(Startup, add_city);
        app.add_systems(Update, (greet_city, find_city, click_in_window));
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GeoQuizz Belgium".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }));
        app.add_plugins(bevy_svg::prelude::SvgPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, camera_zoom_system);
    }
}
