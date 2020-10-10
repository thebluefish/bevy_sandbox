// A one-shot stage
use crate::{resources::{GameState, SubState}, systems::loading_screen};
use bevy::{prelude::*, ecs::Schedule};

pub fn initialize(schedule: &mut Schedule) {
    schedule.add_stage("update");
    schedule.add_system_to_stage("update", run.thread_local_system());
    schedule.add_system_to_stage("update", loading_screen::setup.system());
}

fn run(_world: &mut World, resources: &mut Resources) {
    *resources.get_mut::<GameState>().unwrap() = GameState::Stage3(SubState::Waiting);
}