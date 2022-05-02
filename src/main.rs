mod assets;
mod setup;
mod player;

use bevy::prelude::*;

use assets::AssetsPlugin;
use setup::SetupPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugin(SetupPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetsPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
