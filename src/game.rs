use bevy::{color::palettes::basic::GREEN, prelude::*};
use bevy_svg::prelude::Origin;
use std::path::PathBuf;

use crate::{
    assets::{DEFAULT_BG, DEFAULT_BORDER},
    types::{
        City, CityAssets, CityNameToGuess, GameState, GuessAssets, GuessSet, GuessType, Location,
        Score, ScoreText, SpawnCity, ValidatedGuess, WorldClickCatcher,
    },
};

const MAX_POINTS: u32 = 100;

pub struct InitGamePlugin;

impl Plugin for InitGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnCity>()
            .add_message::<ValidatedGuess>()
            .insert_resource(GameState::Guess)
            .init_resource::<Score>()
            .add_systems(
                Startup,
                (init_guess, trigger_spawn_city.after(init_guess)).chain(),
            );
    }
}

pub fn trigger_spawn_city(mut ev: MessageWriter<SpawnCity>) {
    ev.write(SpawnCity);
}

pub fn init_guess(mut commands: Commands) {
    commands.init_resource::<GuessSet>();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (click_to_spawn_circle, update_button, update_score_text),
        )
        .add_systems(
            Update,
            (
                despawn_city
                    .run_if(on_message::<SpawnCity>)
                    .after(update_button)
                    .chain(),
                evaluate_guess.run_if(on_message::<ValidatedGuess>),
                spawn_city
                    .run_if(on_message::<SpawnCity>)
                    .after(despawn_city)
                    .chain(),
                update_guess_text
                    .run_if(on_message::<SpawnCity>)
                    .after(spawn_city),
            )
                .chain(),
        );
    }
}

fn click_to_spawn_circle(
    mut commands: Commands,
    i: Query<&Interaction, (Changed<Interaction>, With<WorldClickCatcher>)>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    guess_assets: Res<GuessAssets>,
    mut existing_circle: Query<(Entity, &mut Transform), With<GuessType>>,
) {
    let window = windows.single().unwrap();
    if let Ok(interaction) = i.single()
        && *interaction == Interaction::Pressed
        && let Some(cursor_pos) = window.cursor_position()
    {
        let (camera, camera_transform) = camera_q.single().unwrap();
        let world_pos = camera
            .viewport_to_world_2d(camera_transform, cursor_pos)
            .unwrap();
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

fn evaluate_guess(
    guess_query: Query<&GuessType>,
    anwser_query: Query<(&Name, &Location), With<City>>,
    mut reveal_query: Query<&mut Visibility, With<City>>,
    mut score: ResMut<Score>,
) {
    if let Ok((_, loc_field)) = anwser_query.single() {
        let loc = loc_field.0;
        if let Ok(guess) = guess_query.single() {
            match guess {
                GuessType::Location(guess_pos) => {
                    let distance = guess_pos.distance(loc) as u32;
                    let points = MAX_POINTS.saturating_sub(distance);
                    score.total += points;
                    score.max += MAX_POINTS;

                    for mut vis in reveal_query.iter_mut() {
                        *vis = Visibility::Visible;
                    }
                }
                GuessType::Name(_) => todo!(),
            }
        } else {
            info!("No guess has been made yet.");
        }
    }
}

pub fn spawn_city(
    mut commands: Commands,
    mut guess_set: ResMut<GuessSet>,
    asset_server: Res<AssetServer>,
    city_assets: Res<CityAssets>,
) {
    if let Some(city) = guess_set.to_guess.pop() {
        let path: PathBuf = ["fonts", "FiraMono-Medium.ttf"].iter().collect();
        let font: Handle<Font> = asset_server.load(path);
        let location = city.loc.0;
        let name = city.name.as_str();
        info!("spawning {}", name);

        commands.spawn((
            city.clone(),
            Transform::from_translation(location.extend(0.0)),
            Visibility::Hidden,
            children![(
                Mesh2d(city_assets.mesh.clone()),
                MeshMaterial2d(city_assets.material.clone()),
                Visibility::Inherited,
                Transform::default(),
                children![(
                    Text2d::from(name),
                    TextFont {
                        font,
                        font_size: 17.5,
                        ..Default::default()
                    },
                    Visibility::Inherited,
                    // Node {..Default::default()},
                    Transform::from_translation(Vec3 {
                        x: 15.,
                        y: 15.,
                        z: 0.
                    }),
                    Origin::Center,
                )]
            )],
        ));
        info!("city spawned");
    }
}

fn despawn_city(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    query: Query<Entity, With<City>>,
) {
    if *game_state == GameState::Guess {
        error!("Wrong game state");
        return;
    }
    if let Ok(entity) = query.single() {
        info!("Despawning city");
        commands.entity(entity).despawn();
    }
    *game_state = GameState::Guess;
}

fn update_guess_text(
    mut text: Query<&mut TextSpan, With<CityNameToGuess>>,
    name: Single<&Name, With<City>>,
) {
    for mut span in &mut text {
        **span = format! {"{}", name.clone()}
    }
}

fn update_score_text(mut text: Query<&mut TextSpan, With<ScoreText>>, score: Res<Score>) {
    for mut span in &mut text {
        **span = format! {"{}/{}", score.total, score.max}
    }
}

fn update_button(
    interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            Without<WorldClickCatcher>,
        ),
    >,
    mut text_query: Query<&mut Text>,
    mut guess_event: MessageWriter<ValidatedGuess>,
    mut spawn_event: MessageWriter<SpawnCity>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut bg_color, mut border_color, children) in interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *border_color = BorderColor::all(Color::srgb(0.12, 0.4, 0.));
                *bg_color = BackgroundColor(Color::from(GREEN));
                if *game_state == GameState::Guess {
                    **text = "Continue".to_string();
                    *game_state = GameState::Standby;
                    guess_event.write(ValidatedGuess);
                } else {
                    **text = "Confirm".to_string();
                    spawn_event.write(SpawnCity);
                }
            }
            Interaction::Hovered => {
                *border_color = BorderColor::all(Color::srgb(0.12, 0.4, 0.));
                *bg_color = BackgroundColor(DEFAULT_BG);
            }
            Interaction::None => {
                *border_color = BorderColor::all(DEFAULT_BORDER);
                *bg_color = BackgroundColor(DEFAULT_BG);
                if *game_state == GameState::Guess {
                    **text = "Confirm".to_string();
                } else {
                    **text = "Continue".to_string();
                }
            }
        }
    }
}
