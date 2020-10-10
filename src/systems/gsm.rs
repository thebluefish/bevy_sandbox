use crate::resources::*;
use super::states;
use bevy::{prelude::*, ecs::{Schedule, ThreadLocalExecution, SystemId, TypeAccess, ArchetypeAccess, ParallelExecutor}};
use std::borrow::Cow;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GameState::Stage1(SubState::Done))
            .add_resource(GameTick::default())
            .add_stage_before("pre_update", "game_update")
            .add_system_to_stage("game_update", Box::new(GameStateManager::new()))
        ;
    }
}

// Packs the schedule and executor together
pub struct StateSchedule(pub Schedule, pub ParallelExecutor);

impl Default for StateSchedule {
    fn default() -> Self {
        StateSchedule(Default::default(), ParallelExecutor::without_tracker_clears())
    }
}

impl StateSchedule {
    fn run(&mut self, mut world: &mut World, mut resources: &mut Resources) {
        self.0.initialize(world, resources);
        self.1.run(&mut self.0, &mut world, &mut resources);
    }
}

// The GSM is a System that runs one of multiple states
// It will run the Game state at a fixed rate as specified by the GameTick Resource
pub struct GameStateManager {
    pub resource_access: TypeAccess,
    pub id: SystemId,
    pub archetype_access: ArchetypeAccess,
    pub acc: f64, // accumulator for fixed game ticks
    pub stage_1: StateSchedule, // Loads the loading screen
    pub stage_2: StateSchedule, // Shows the loading screen
    pub stage_3: StateSchedule, // Loads the rest of our assets
    pub game_start: StateSchedule, // Hides the loading screen, preps game state
    pub game: StateSchedule, // Runs game simulation at a fixed rate
}

impl GameStateManager {
    pub fn new() -> Self {
        GameStateManager {
            resource_access: Default::default(),
            id: SystemId::new(),
            archetype_access: Default::default(),
            acc: 0.0,
            stage_1: Default::default(),
            stage_2: Default::default(),
            stage_3: Default::default(),
            game_start: Default::default(),
            game: Default::default()
        }
    }
}

impl System for GameStateManager {
    fn name(&self) -> Cow<'static, str> { "GSM".into() }

    fn id(&self) -> SystemId { self.id }

    fn update_archetype_access(&mut self, _world: &World) { self.archetype_access.clear(); }

    fn archetype_access(&self) -> &ArchetypeAccess { &self.archetype_access }

    fn resource_access(&self) -> &TypeAccess { &self.resource_access }

    fn thread_local_execution(&self) -> ThreadLocalExecution { ThreadLocalExecution::Immediate }

    fn run(&mut self, _world: &World, _resources: &Resources)  { }

    fn run_thread_local(&mut self, mut world: &mut World, mut resources: &mut Resources) {
        let state = *resources.get::<GameState>().unwrap(); // copy the state to avoid borrow issues

        match state {
            GameState::Stage1(_) => self.stage_1.run(&mut world, &mut resources),
            GameState::Stage2 => self.stage_2.run(&mut world, &mut resources),
            GameState::Stage3(_) => self.stage_3.run(&mut world, &mut resources),
            GameState::GameStart => self.game_start.run(&mut world, &mut resources),
            GameState::Game => {
                let rate = match (resources.get::<Time>(), resources.get::<GameTick>()) {
                    (Some(time), Some(rate)) => {
                        self.acc += time.delta_seconds_f64;
                        Some(rate.rate)
                    },
                    _ => { None },
                };

                // We had to capture the rate to avoid borrow issues
                match rate {
                    Some(rate) => {
                        // Run the schedule at fixed interval
                        while self.acc >= rate {
                            self.game.run(world, resources);
                            self.acc -= rate;
                        }
                    },
                    _ => {},
                }
            },
        }
    }

    fn initialize(&mut self, _world: &mut World, _resources: &mut Resources) {
        states::stage_1::initialize(&mut self.stage_1.0);
        states::stage_2::initialize(&mut self.stage_2.0);
        states::stage_3::initialize(&mut self.stage_3.0);
        states::game_start::initialize(&mut self.game_start.0);
        states::game::initialize(&mut self.game.0);
    }
}