use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_svg::prelude::Origin;
use geo_quizz::{camera, load_database};

use geo_quizz::types::{BundleCity, City, Location, Name};

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(camera::MapPlugin)
        .run();
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
pub struct CityAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

fn setup_city_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(2.5));
    let material = materials.add(Color::srgb(255., 0., 0.));
    commands.insert_resource(CityAssets { mesh, material });
}

#[derive(Resource)]
struct LocTimer(Timer);

fn add_city(mut commands: Commands) {
    let path = std::path::Path::new(r"..\database\belgium_cities.json");
    let db = load_database(path);
    println!("{db:?}");
    commands.spawn(BundleCity {
        city: City,
        name: Name("Soignies".to_string()),
        loc: Location(Vec2::new(-89.0, 13.0)),
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
        if event.button == MouseButton::Left
            && event.state.is_pressed()
            && let Some(cursor_pos) = window.cursor_position()
        {
            let (camera, camera_transform) = camera_q.single().unwrap();
            let world_pos = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap();
            println!("Click at {:?}", world_pos);
            if let Ok((entity, mut transform)) = existing_circle.single_mut() {
                // Move the existing circle
                transform.translation = world_pos.extend(0.0);
                commands
                    .entity(entity)
                    .insert(GuessType::Location(world_pos));
            } else {
                commands.spawn((
                    Mesh2d(guess_assets.mesh.clone()),
                    MeshMaterial2d(guess_assets.material.clone()),
                    Transform::from_translation(world_pos.extend(0.0)),
                    GuessType::Location(world_pos),
                ));
            }
        };
    }
}

fn spawn_city_with_label(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    city_assets: Res<CityAssets>,
    city: BundleCity,
) {
    let font: Handle<Font> = asset_server.load(r"fonts\FiraMono-Medium.ttf");
    let location = city.loc.0;
    let name = city.name.0.clone();
    // Spawn the city point
    commands.spawn((
        Mesh2d(city_assets.mesh.clone()),
        MeshMaterial2d(city_assets.material.clone()),
        Transform::from_translation(location.extend(0.0)),
    ));
    // Spawn the text above the point
    commands.spawn((
        Text2d::from(name),
        TextFont {
            font,
            font_size: 17.5,
            ..Default::default()
        },
        // Node {..Default::default()},
        Transform::from_translation((location + Vec2 { x: 15., y: 15. }).extend(1.0)),
        Origin::Center,
    ));
}

fn evaluate_guess(
    commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    guess_query: Query<&GuessType>,
    anwser_query: Query<(Entity, &Name, &Location), With<City>>,
    asset_server: Res<AssetServer>,
    city_assets: Res<CityAssets>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok((_entity, name_field, loc_field)) = anwser_query.single() {
            let name = &name_field.0;
            let loc = loc_field.0;
            if let Ok(guess) = guess_query.single() {
                match guess {
                    GuessType::Location(guess_pos) => {
                        let distance = guess_pos.distance(loc);
                        println!("{} is {:.1} a.u. away from your guess!", name, distance);
                        spawn_city_with_label(
                            commands,
                            asset_server,
                            city_assets,
                            BundleCity {
                                city: City,
                                name: Name(name.clone()),
                                loc: Location(loc),
                            },
                        );
                    }
                    GuessType::Name(_) => todo!(),
                }
            } else {
                println!("No guess has been made yet.");
            }
            // commands.entity(entity).despawn();
        } else {
            println!("Multiple answers possible")
        }
    }
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_guess_assets, setup_city_assets, add_city));
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
