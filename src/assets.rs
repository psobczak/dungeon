use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{prelude::*, sprite::Rect, utils::HashMap};

const SPRITES_COORDINATES: &str = "assets/tiles_list_v1.4";
const SPRITE_SHEET_FILE: &str = "0x72_DungeonTilesetII_v1.4.png";

pub struct AssetsPlugin;

pub struct SpriteSheet(pub Handle<TextureAtlas>);

pub struct SpriteIndices(pub HashMap<String, usize>);

#[derive(Debug)]
pub struct Coordinates {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    columns: Option<f32>,
}

pub struct SpritesCoordinates(pub HashMap<String, Coordinates>);

impl From<&str> for Coordinates {
    fn from(value: &str) -> Self {
        let coordinates: Vec<f32> = value
            .split_ascii_whitespace()
            .into_iter()
            .map(|c| c.parse::<f32>().unwrap())
            .collect();

        Self {
            x: *coordinates.get(0).unwrap(),
            y: *coordinates.get(1).unwrap(),
            width: *coordinates.get(2).unwrap(),
            height: *coordinates.get(3).unwrap(),
            columns: Option::<&f32>::cloned(coordinates.get(4)),
        }
    }
}

impl SpritesCoordinates {
    fn new() -> Self {
        let mut coordinates = HashMap::<String, Coordinates>::new();
        let file = File::open(SPRITES_COORDINATES).unwrap();
        let lines = BufReader::new(file).lines();

        for line in lines {
            if let Ok(value) = line {
                if !value.is_empty() {
                    let (key, value) = text_to_coordinates(value);
                    coordinates.insert(key, value);
                }
            }
        }

        SpritesCoordinates(coordinates)
    }
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpritesCoordinates::new())
            .add_startup_system_to_stage(StartupStage::PreStartup, loads_sprites);
    }
}

fn text_to_coordinates(input: String) -> (String, Coordinates) {
    let (prefix, suffix) = input.split_once(' ').unwrap();
    let coordinates = Coordinates::from(suffix);
    (prefix.to_string(), coordinates)
}

fn loads_sprites(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    coordinates: Res<SpritesCoordinates>,
) {
    let image = assets.load(SPRITE_SHEET_FILE);
    let mut atlas = TextureAtlas::new_empty(image, Vec2::splat(512.0));

    let mut indices = SpriteIndices(HashMap::<String, usize>::new());

    coordinates.0.iter().for_each(|(name, c)| {
        let columns = c.columns.unwrap_or(1.0);
        let rect = Rect {
            min: Vec2::new(c.x, c.y),
            max: Vec2::new((columns * c.width) + c.x, c.y + c.height),
        };

        let texture_index = atlas.add_texture(rect);
        indices.0.insert(name.to_string(), texture_index);
    });

    commands.insert_resource(SpriteSheet(texture_atlas.add(atlas)));

    commands.insert_resource(indices);
}
