use bevy::prelude::*;

use crate::assets::{SpriteIndices, SpriteSheet};

const PLAYER_SPEED: f32 = 50.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement);
    }
}

fn spawn_player(mut commands: Commands, sprites: Res<SpriteSheet>, indices: Res<SpriteIndices>) {
    let sprite = get_texture_sprite_image("weapon_bow", indices);
    commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas: sprites.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 999.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player {
        speed: PLAYER_SPEED
    })
    .insert(Name::new("Player"));
}

fn get_texture_sprite_image(name: &str, indices: Res<SpriteIndices>) -> TextureAtlasSprite {
    match indices.0.get(name) {
        Some(index) => TextureAtlasSprite::new(*index),
        None => panic!("Could not find sprite with given name or index"),
    }
}

fn player_movement(mut player_query: Query<(&Player, &mut Transform)>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    let (player, mut transform) = player_query.single_mut();
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * time.delta_seconds()
    }

    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.speed * time.delta_seconds()
    }

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += player.speed * time.delta_seconds()
    }

    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * time.delta_seconds()
    }
}
