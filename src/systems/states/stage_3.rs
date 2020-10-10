use crate::{assets::*, resources::{GameState, SubState}};
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
            GameState::Stage3(stage) => {
                *stage = SubState::Done;
            },
            _ => { panic!("running on wrong stage!")}
        }

    }
}

fn end(_world: &mut World, resources: &mut Resources) {

    let mut state = resources.get_mut::<GameState>().unwrap();

    let done = match &mut *state {
        GameState::Stage3(SubState::Done) => {
            println!("Stage3 Done!");
            true
        },
        GameState::Stage3(SubState::Waiting) => {
            // println!("Stage3 Waiting!");
            false
        },
        _ => { panic!("running on wrong stage!")}
    };

    if done {
        *state = GameState::GameStart;
    }
}

fn run(_world: &mut World, resources: &mut Resources) {
    let mut all_loaded = true;

    if !GameTextures::check_all(resources) { all_loaded = false }
    if !GameFonts::check_all(resources) { all_loaded = false }

    if all_loaded == false {
        *resources.get_mut::<GameState>().unwrap() = GameState::Stage3(SubState::Waiting);
    }
}
