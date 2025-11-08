use crate::types::{CityAssets, CityNameToGuess, GuessAssets, Score, ScoreText, WorldClickCatcher};
use bevy::{
    color::palettes::basic::{BLACK, RED},
    prelude::*,
    ui::FocusPolicy,
};
use std::path::PathBuf;

pub const DEFAULT_BORDER: Color = Color::BLACK;
pub const DEFAULT_BG: Color = Color::srgb(0.15, 0.15, 0.15);

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_guess_assets,
                setup_city_assets,
                setup_texts,
                setup_button,
            ),
        );
    }
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

fn setup_city_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(2.5));
    let material = materials.add(Color::srgb(255., 0., 0.));
    commands.insert_resource(CityAssets { mesh, material });
}

fn setup_texts(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            // `Text` or `Text2d` are needed, and will provide default instances
            // of the following components.
            Text::new("Find "),
            TextColor(BLACK.into()),
        ))
        .with_child((
            // Children must be `TextSpan`, not `Text` or `Text2d`.
            TextSpan::default(),
            CityNameToGuess,
            TextColor(RED.into()),
        ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(5.),
                ..default()
            },
            Text::new("Score: "),
            TextColor(BLACK.into()),
        ))
        .with_child((
            // Children must be `TextSpan`, not `Text` or `Text2d`.
            TextSpan::new(format!("{}/{}", score.total, score.max)),
            ScoreText,
            TextColor(RED.into()),
        ));
}

fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![(
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
            WorldClickCatcher,
            FocusPolicy::Pass,
            children![button(&asset_server)]
        )],
    ));
}

fn button(asset_server: &AssetServer) -> impl Bundle + use<> {
    (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(5.),
            bottom: Val::Percent(10.),
            // width: Val::Percent(100.0),
            // height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(75.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            FocusPolicy::Block,
            children![(
                Text::new("Confirm"),
                TextFont {
                    font: asset_server
                        .load(["fonts", "FiraMono-Medium.ttf"].iter().collect::<PathBuf>()),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )]
        )],
    )
}
