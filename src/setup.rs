use bevy::{prelude::*, window::PresentMode};

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 600.0;

const CLEAR_COLOR: Color = Color::rgb(200.0, 100.0, 100.0);

pub struct SetupPlugin;
pub struct WindowSize {
    height: f32,
    width: f32,
}

impl WindowSize {
    pub fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height / 2.0
    }
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CLEAR_COLOR)
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "DUNGEON".to_string(),
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .insert_resource(WindowSize {
            height: WINDOW_HEIGHT,
            width: WINDOW_WIDTH,
        })
        .add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
