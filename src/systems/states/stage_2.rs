// A one-shot stage
use crate::{assets, resources::{GameState, Loader}, systems::loading_screen};
use bevy::{prelude::*, ecs::Schedule};

pub fn initialize(schedule: &mut Schedule) {
    schedule.add_stage("update");
    schedule.add_system_to_stage("update", run.system());
    schedule.add_system_to_stage("update", loading_screen::setup.system());
}

fn run(
    asset_server: Res<AssetServer>,
    mut state: ResMut<GameState>,
    mut loader: ResMut<Loader>,
) {
    // Fonts
    loader.load::<Font, _>(&asset_server, &[
        crate::assets::FONTS_MINI_SQUARE,
        crate::assets::FONTS_PIXEL_SQUARE,
    ]).unwrap();

    // Textures
    loader.load::<Texture, _>(&asset_server, &[
        assets::TEXTURES_ROGUELIKE_CHAR,
        assets::TEXTURES_ROGUELIKE_DUNGEON,
        assets::TEXTURES_ROGUELIKE_SHEET,
        assets::TEXTURES_UIPACK_SHEET,
    ]).unwrap();

    *state = GameState::Stage3;
}