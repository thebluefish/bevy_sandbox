use crate::{assets, resources::{GameState, Loader}};
use bevy::{prelude::*, ecs::Schedule};

pub fn initialize(schedule: &mut Schedule) {
    schedule.add_stage("update");
    schedule.add_system_to_stage("update", run.system());
}

fn run(
    asset_server: Res<AssetServer>,
    mut state: ResMut<GameState>,
    mut loader: ResMut<Loader>,
) {
    loader.load::<Texture, _>(&asset_server, &[assets::LOADING]).unwrap();
    *state = GameState::Stage1;
}