use crate::{resources::{Loader, GameState}};
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
    if loader.check(&asset_server) {
        println!("Preload done...");
        *state = GameState::Stage2;
    }
}
