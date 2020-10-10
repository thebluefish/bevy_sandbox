use crate::{assets::GameAssets, resources::{GameState, SubState}};
use bevy::{prelude::*, ecs::Schedule};

pub fn initialize(schedule: &mut Schedule) {
    schedule.add_stage("update");
    schedule.add_system_to_stage("update", begin.thread_local_system());
    schedule.add_system_to_stage("update", run.thread_local_system());
    schedule.add_system_to_stage("update", end.thread_local_system());
}

fn begin(_world: &mut World, resources: &mut Resources) {
    {
        let mut state = resources.get_mut::<GameState>().unwrap();

        match &mut *state {
            GameState::Stage1(stage) => {
                *stage = SubState::Done;
            },
            _ => { panic!("running on wrong stage!")}
        }

    }
}

fn end(_world: &mut World, resources: &mut Resources) {

    let mut state = resources.get_mut::<GameState>().unwrap();

    let done = match &mut *state {
        GameState::Stage1(SubState::Done) => {
            println!("Stage1 Done!");
            true
        },
        GameState::Stage1(SubState::Waiting) => {
            // println!("Stage1 Waiting!");
            false
        },
        _ => { panic!("running on wrong stage!")}
    };

    if done {
        *state = GameState::Stage2;
    }
}

fn run(_world: &mut World, resources: &mut Resources) {
    let mut all_loaded = true;

    if !GameAssets::check_all(resources) { all_loaded = false }

    if all_loaded == false {
        *resources.get_mut::<GameState>().unwrap() = GameState::Stage1(SubState::Waiting);
    }
}
