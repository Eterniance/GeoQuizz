use bevy::prelude::*;
use bevy_svg::prelude::Origin;
use std::path::PathBuf;

use bevy::input::mouse::MouseButtonInput;

use crate::types::{
    BundleCity, City, CityAssets, GuessAssets, GuessSet, GuessType, GuessValidated, Location,
    SpawnCity,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (click_to_spawn_circle, trigger_guess_validation))
            .add_systems(
                Update,
                (evaluate_guess, spawn_city.after(evaluate_guess)).chain(),
            );
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

fn reveal_city(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    city_assets: Res<CityAssets>,
    city: BundleCity,
) {
    let path: PathBuf = ["fonts", "FiraMono-Medium.ttf"].iter().collect();
    let font: Handle<Font> = asset_server.load(path);
    let location = city.loc.0;
    let name = city.name.as_str();
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
    mut commands: Commands,
    mut events: EventReader<GuessValidated>,
    mut spawn_city_event: EventWriter<SpawnCity>,
    guess_query: Query<&GuessType>,
    anwser_query: Query<(Entity, &Name, &Location), With<City>>,
    asset_server: Res<AssetServer>,
    city_assets: Res<CityAssets>,
) {
    if events.read().last().is_some() {
        if let Ok((entity, name_field, loc_field)) = anwser_query.single() {
            commands.entity(entity).despawn();
            let name = name_field.as_str();
            let loc = loc_field.0;
            if let Ok(guess) = guess_query.single() {
                match guess {
                    GuessType::Location(guess_pos) => {
                        let distance = guess_pos.distance(loc);
                        println!("{} is {:.1} a.u. away from your guess!", name, distance);
                        reveal_city(
                            commands,
                            asset_server,
                            city_assets,
                            BundleCity {
                                city: City,
                                name: Name::new(name.to_string()),
                                loc: Location(loc),
                            },
                        );
                    }
                    GuessType::Name(_) => todo!(),
                }
            } else {
                println!("No guess has been made yet.");
            }
        } else {
            println!("Multiple answers possible")
        }
        spawn_city_event.write(SpawnCity);
    }
}

pub fn spawn_city(
    mut commands: Commands,
    mut guess_set: ResMut<GuessSet>,
    mut events: EventReader<SpawnCity>,
) {
    if events.read().last().is_some()
        && let Some(city) = guess_set.to_guess.pop()
    {
        println!("Find {}", city.name);
        commands.spawn(city.clone());
    }
}

pub fn trigger_guess_validation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<GuessValidated>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event.write(GuessValidated);
    }
}

pub struct InitGame;

impl Plugin for InitGame {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCity>()
            .add_event::<GuessValidated>()
            .add_systems(
                Startup,
                (init_guess, trigger_spawn_city.after(init_guess)).chain(),
            );
    }
}

pub fn trigger_spawn_city(mut event: EventWriter<SpawnCity>) {
    event.write(SpawnCity);
}

pub fn init_guess(mut commands: Commands) {
    commands.init_resource::<GuessSet>();
}
