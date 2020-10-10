use bevy::ecs::Schedule;

pub fn initialize(schedule: &mut Schedule) {
    schedule.add_stage("update");
}