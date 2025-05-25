mod camera;

use bevy::prelude::*;
use bevy::input::{gestures::*,
        mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel}};


// mod game;

fn main() {
    App::new()
        .add_plugins(camera::MapPlugin)
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


#[derive(Component)]
struct GuessMarker;

fn click_to_spawn_circle(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut existing_circle_q: Query<(Entity, &mut Transform), With<GuessMarker>>,
) {
    let window = windows.single().unwrap();

    let circle = Mesh2d(meshes.add(Circle::new(2.5)));
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert screen pos to world coordinates
                let Ok((camera, camera_transform)) = camera_q.single() else { todo!() };
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        if let Ok((_, mut transform)) = existing_circle_q.single_mut() {
                            // Move the existing circle
                            transform.translation = world_pos.extend(0.0);
                        } else {
                            commands.spawn((
                                circle.clone(),
                                MeshMaterial2d(materials.add(Color::WHITE)),
                                Transform::from_translation(world_pos.extend(0.0)),
                                GuessMarker,
                        ));
                    }
                } else {todo!()};
            };
        }
    }
}



pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Once)));
        app.insert_resource(LocTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        app.add_systems(Startup, add_city);
        app.add_systems(Update, (greet_city, find_city, click_to_spawn_circle));
    }
}

