mod camera;

use bevy::input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

// mod game;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(camera::MapPlugin)
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
pub enum GuessType {
    Name(String),
    Location(Vec2),
}

#[derive(Resource)]
pub struct GuessAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

fn setup_guess_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(2.5));
    let material = materials.add(Color::WHITE);

    commands.insert_resource(GuessAssets { mesh, material });
}


#[derive(Resource)]
struct LocTimer(Timer);

fn add_city(mut commands: Commands) {
    commands.spawn(BundleCity {
        city: City,
        name: Name("Soignies".to_string()),
        loc: Location(Vec2::new(0.0, 0.0)),
    });
}

fn find_city(time: Res<Time>, mut timer: ResMut<LocTimer>, query: Query<&Location, With<City>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for loc in &query {
            println!("At {:?}", loc.0);
        }
    }
}

fn click_to_spawn_circle(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    guess_assets: Res<GuessAssets>,
    mut existing_circle: Query<(Entity, &mut Transform), With<GuessType>>,
) {
    let window = windows.single().unwrap();
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            if let Some(cursor_pos) = window.cursor_position() {
                let (camera, camera_transform) = camera_q.single().unwrap();
                let world_pos = camera.viewport_to_world_2d(camera_transform, cursor_pos).unwrap();
                    if let Ok((entity, mut transform)) = existing_circle.single_mut() {
                        // Move the existing circle
                        transform.translation = world_pos.extend(0.0);
                        commands.entity(entity).insert(GuessType::Location(world_pos));
                    } else {
                        commands.spawn((
                            Mesh2d(guess_assets.mesh.clone()),
                            MeshMaterial2d(guess_assets.material.clone()),
                            Transform::from_translation(world_pos.extend(0.0)),
                            GuessType::Location(world_pos),
                        ));
                    }
                };
            };
        }
    }

fn evaluate_guess(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    guess_query: Query<&GuessType>,
    anwser_query: Query<(&Name, &Location), With<City>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok( (true_name, true_loc) ) = anwser_query.single() {
            if let Ok(guess) = guess_query.single() {
                match guess {
                    GuessType::Location(pos) => {
                        let distance = pos.distance(true_loc.0);
                        println!("{} is {:.1} a.u. away from your guess!", true_name.0, distance);
                    }
                    GuessType::Name(_) => todo!(),
                }
            } else {
                println!("No guess has been made yet.");
            }
        }
    }
}



pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_guess_assets, add_city));
        app.add_systems(Update, (click_to_spawn_circle, evaluate_guess));
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LocTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        app.add_systems(Startup, add_city);
        app.add_systems(Update, (find_city, click_to_spawn_circle));
    }
}
