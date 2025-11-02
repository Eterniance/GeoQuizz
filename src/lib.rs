pub mod camera;
use bevy::app::{PluginGroup, PluginGroupBuilder};
pub use camera::MapPlugin;

mod loader;
pub use loader::*;

pub mod types;

mod assets;
pub use assets::AssetsPlugin;

mod game;
pub use game::GamePlugin;

use crate::game::InitGame;

pub struct SetupPlugin;

impl PluginGroup for SetupPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetsPlugin)
            .add(MapPlugin)
            .add(InitGame)
    }
}
