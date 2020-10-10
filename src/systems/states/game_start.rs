use bevy::{prelude::*, ecs::Schedule};
use crate::systems::loading_screen;

pub fn initialize(schedule: &mut Schedule) {
    schedule.add_stage("update");
    schedule.add_system_to_stage("update", loading_screen::teardown.system());
}